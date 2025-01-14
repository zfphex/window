use crate::*;
use crossbeam_queue::SegQueue;
use std::pin::Pin;

#[derive(Debug)]
pub struct Window {
    pub hwnd: isize,
    pub screen_mouse_pos: (i32, i32),

    //TODO: Remove, this is super overkill.
    //The only events going through this now are Quit and Dpi.
    //I think an array or vec with small capacity would be fine.
    //I do like that it has interior mutability since it's atomic.
    pub queue: SegQueue<Event>,
}

impl Window {
    pub fn scale_factor(&self) -> f32 {
        const DEFAULT_DPI: f32 = 96.0;
        unsafe { GetDpiForWindow(self.hwnd) as f32 / DEFAULT_DPI }
    }
    pub fn dpi(&self) -> u32 {
        unsafe { GetDpiForWindow(self.hwnd) }
    }
    pub fn client_area(&self) -> RECT {
        client_area(self.hwnd)
    }
    pub fn screen_area(&self) -> RECT {
        screen_area(self.hwnd)
    }
    pub fn borderless(&mut self) {
        unsafe {
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE) as isize);

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

    pub fn set_pos(&self, x: i32, y: i32) {
        unsafe {
            SetWindowPos(self.hwnd, 0, x, y, 0, 0, SWP_FRAMECHANGED | SWP_NOSIZE);
        }
    }

    pub fn reset_style(&mut self) {
        unsafe {
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, DEFAULT_WINDOW_STYLE as isize);

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
            window.queue.push(Event::Dpi(wparam));
            return 1;
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

pub const DEFAULT_WINDOW_STYLE: u32 =
    WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_VISIBLE;

pub const WINDOW_BORDERLESS: u32 = WS_POPUP | WS_VISIBLE;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum WindowStyle {
    Default =
        WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_VISIBLE,
    Borderless = WS_POPUP | WS_VISIBLE,
    //TODO: WS_EX_TOPMOST
}

impl WindowStyle {
    pub const fn as_u32(self) -> u32 {
        unsafe { std::mem::transmute::<WindowStyle, u32>(self) }
    }
}

pub fn create_window(
    title: &str,
    // x: Option<i32>,
    // y: Option<i32>,
    width: i32,
    height: i32,
    //TODO: We need bitflags so the user can customize however they like.
    //There are also extended flags that need to be handled seperately.
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
            0,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            style.as_u32(),
            // DEFAULT_WINDOW_STYLE,
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

        assert_ne!(hwnd, 0);
        WINDOW_COUNT.fetch_add(1, Ordering::SeqCst);

        //Safety: This *should* be pinned.
        let window = Box::pin(Window {
            hwnd,
            screen_mouse_pos: (0, 0),
            queue: SegQueue::new(),
        });

        let addr = &*window as *const Window;
        let result = SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, addr as isize);
        assert!(result <= 0);

        window
    }
}
