use crate::*;
use crossbeam_queue::SegQueue;
use std::pin::Pin;

pub const DEFAULT_DPI: f32 = 96.0;

#[derive(Debug)]
pub struct Window {
    pub hwnd: isize,
    pub screen_mouse_pos: (i32, i32),
    //TODO:
    // pub display_scaling: AtomicF32
    pub display_scale: f32,

    //TODO: Remove, this is super overkill.
    //The only events going through this now are Quit and Dpi.
    //I think an array or vec with small capacity would be fine.
    //I do like that it has interior mutability since it's atomic.
    pub queue: SegQueue<Event>,
}

impl Window {
    ///Updates the width and height based on the display scale.
    pub fn rescale_window(&self) {
        let area = client_area(self.hwnd);
        let (width, height) = if self.display_scale == 1.0 {
            (
                area.width() as f32 / self.display_scale,
                area.height() as f32 / self.display_scale,
            )
        } else {
            (
                area.width() as f32 * self.display_scale,
                area.height() as f32 * self.display_scale,
            )
        };

        unsafe {
            SetWindowPos(
                self.hwnd,
                0,
                area.left,
                area.top,
                width as i32,
                height as i32,
                SWP_FRAMECHANGED,
            )
        };
    }
    pub fn client_area(&self) -> RECT {
        client_area(self.hwnd)
    }
    pub fn screen_area(&self) -> RECT {
        screen_area(self.hwnd)
    }
    pub fn borderless(&mut self) {
        unsafe {
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, WindowStyle::BORDERLESS.style as isize);

            //Update the window area without moving or resizing it.
            SetWindowPos(
                self.hwnd,
                0,
                0,
                0,
                0,
                0,
                SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
            );
        };
    }

    pub fn move_window(&self, x: i32, y: i32) {
        let area = client_area(self.hwnd);
        unsafe { MoveWindow(self.hwnd, x, y, area.width(), area.height(), 0) };
    }

    pub fn set_pos(&self, x: i32, y: i32, cx: i32, cy: i32) {
        unsafe {
            SetWindowPos(self.hwnd, 0, x, y, cx, cy, SWP_FRAMECHANGED);
        }
    }

    pub fn reset_style(&mut self) {
        unsafe {
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, WindowStyle::DEFAULT.style as isize);

            //Update the window area without moving or resizing it.
            SetWindowPos(
                self.hwnd,
                0,
                0,
                0,
                0,
                0,
                SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
            );
        };
    }
    //TODO:
    pub fn draw(&self, _buffer: &[u32], _bitmap: BITMAPINFO) {
        // unsafe {
        //     StretchDIBits(
        //         self.context,
        //         0,
        //         0,
        //         self.width,
        //         self.height,
        //         0,
        //         0,
        //         self.width,
        //         self.height,
        //         buffer.as_mut_ptr() as *const c_void,
        //         &bitmap,
        //         0,
        //         SRCCOPY,
        //     )
        // };
    }
    pub fn event(&self) -> Option<Event> {
        //Window procedure events take presidence here.
        if let Some(event) = self.queue.pop() {
            return Some(event);
        };

        event(Some(self.hwnd))
    }
}

pub unsafe extern "system" fn wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    if msg == WM_CREATE {
        set_dark_mode(hwnd).unwrap();
        return 0;
    }

    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window;
    if ptr.is_null() {
        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    //I'm not convinced this is the right way to do this.
    let window: &mut Window = &mut *ptr;

    match msg {
        WM_DESTROY | WM_CLOSE => {
            window.queue.push(Event::Quit);
            return 0;
        }
        //https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-getdpiscaledsize
        WM_GETDPISCALEDSIZE => {
            window.display_scale = wparam as f32 / DEFAULT_DPI;
            window.queue.push(Event::Dpi(wparam));
            return 1;
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

// pub const TOP: u32 = WS_EX_TOPMOST;

pub struct WindowStyle {
    pub style: u32,
    pub exstyle: u32,
}

impl WindowStyle {
    pub const DEFAULT: Self = Self {
        style: WS_CAPTION
            | WS_SYSMENU
            | WS_THICKFRAME
            | WS_MINIMIZEBOX
            | WS_MAXIMIZEBOX
            | WS_VISIBLE,
        exstyle: 0,
    };
    pub const BORDERLESS: Self = Self {
        style: WS_POPUP | WS_VISIBLE,
        exstyle: 0,
    };

    pub fn ex_style(mut self, flags: u32) -> Self {
        self.exstyle |= flags;
        self
    }

    pub fn style(mut self, flags: u32) -> Self {
        self.style |= flags;
        self
    }
}

pub fn create_window(
    title: &str,
    // x: Option<i32>,
    // y: Option<i32>,
    width: i32,
    height: i32,
    style: WindowStyle,
) -> Pin<Box<Window>> {
    unsafe {
        if SetThreadDpiAwarenessContext(DpiAwareness::MonitorAwareV2) == 0 {
            panic!("Only Windows 10 (1607) or later is supported.")
        };

        //Title must be null terminated.
        let title = std::ffi::CString::new(title).unwrap();

        let wnd_class = WNDCLASSA {
            style: 0,
            wnd_proc: Some(wnd_proc),
            cls_extra: 0,
            wnd_extra: 0,
            instance: 0,
            icon: 0,
            //Prevent cursor from changing when loading.
            cursor: LoadCursorW(null_mut(), IDC_ARROW) as isize,
            background: 0,
            menu_name: core::mem::zeroed(),
            class_name: title.as_ptr() as *const u8,
        };

        let _ = RegisterClassA(&wnd_class);

        //Imagine that the users wants a window that is 800x600.
        //`CreateWindow` takes in screen coordinates instead of client coordiantes.
        //Which means that it will set the window size including the title bar and borders etc.
        //We must convert the requested client coordinates to screen coordinates.

        //TODO: What is this value at different DPI's?
        // const WINDOW_PADDING_96_DPI: i32 = 7;
        //         let rect = RECT {
        //             left: x - WINDOW_PADDING_96_DPI,
        //             top: y,
        //             right: x + width,
        //             bottom: y + height,
        //         };

        let hwnd = CreateWindowExA(
            style.exstyle,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            style.style,
            // WindowStyle::DEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            // rect.left,
            // rect.top,
            // rect.width(),
            // rect.height(),
            0,
            0,
            0,
            null(),
        );

        let display_scale = GetDpiForWindow(hwnd) as f32 / DEFAULT_DPI;

        //There is no way to know which monitor the window will be on and what DPI it will have before creation.
        //We then need to scale the window after creation.
        if display_scale != 1.0 {
            let area = client_area(hwnd);
            SetWindowPos(
                hwnd,
                0,
                area.left,
                area.top,
                (area.width() as f32 * display_scale) as i32,
                (area.height() as f32 * display_scale) as i32,
                SWP_FRAMECHANGED,
            );
        }

        assert_ne!(hwnd, 0);
        WINDOW_COUNT.fetch_add(1, Ordering::SeqCst);

        //Safety: This *should* be pinned.
        let window = Box::pin(Window {
            display_scale,
            hwnd,
            screen_mouse_pos: (0, 0),
            queue: SegQueue::new(),
        });

        if display_scale == 1.0 {
            window.rescale_window();
        }

        let addr = &*window as *const Window;
        let result = SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, addr as isize);
        assert!(result <= 0);

        window
    }
}
