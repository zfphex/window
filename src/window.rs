use crate::*;
pub const DEFAULT_DPI: f32 = 96.0;

pub fn create_window(
    title: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    style: WindowStyle,
) -> std::pin::Pin<Box<Window>> {
    unsafe {
        if SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) == 0 {
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

        //Adjust the rect to fit exactly what the user requested.
        //Windows has padding and other weird nonsense when trying set the width and height.
        //Not needed anymore?

        // let mut rect = RECT {
        //     left: 0,
        //     top: 0,
        //     right: width as i32,
        //     bottom: height as i32,
        // };
        // AdjustWindowRectEx(&mut rect, style.style, 0, style.exstyle);

        RegisterClassA(&wnd_class);

        let hwnd = CreateWindowExA(
            style.exstyle,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            style.style,
            if x == 0 { CW_USEDEFAULT } else { x },
            if y == 0 { CW_USEDEFAULT } else { y },
            // CW_USEDEFAULT,
            // CW_USEDEFAULT,
            //These are adjusted later for DPI scaling.
            width,
            height,
            0,
            0,
            0,
            null(),
        );

        //Get the display scale factor 1.0, 1.25, 1.5, 1.75, can also be custom.
        let scale = GetDpiForWindow(hwnd) as f32 / DEFAULT_DPI;
        let mut area = get_client_rect(hwnd);

        //Scale the size of the window to match the display scale.
        //AdjustWindowRect used to be needed, but isn't anymore, I'm not sure why?
        if scale != 1.0 {
            SetWindowPos(
                hwnd,
                0,
                area.x as i32,
                area.y as i32,
                (area.width as f32 * scale) as i32,
                (area.height as f32 * scale) as i32,
                SWP_FRAMECHANGED,
            );
            //Update the area since SetWindowPos will change it.
            area = get_client_rect(hwnd);
        }

        assert_ne!(hwnd, 0);
        let dc = GetDC(hwnd);

        //Safety: This *should* be pinned.
        let window = Box::pin(Window {
            //Re-grab the area after calling SetWindowPos.
            area,
            hwnd,
            dc,
            display_scale: scale,
            buffer: vec![0u32; area.width * area.height],
            bitmap: BITMAPINFO::new(area.width as i32, area.height as i32),
            quit: false,
            mouse_position: Rect::default(),
            left_mouse: MouseButtonState::new(),
            right_mouse: MouseButtonState::new(),
            middle_mouse: MouseButtonState::new(),
            mouse_4: MouseButtonState::new(),
            mouse_5: MouseButtonState::new(),
        });

        let addr = &*window as *const Window;
        let result = SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, addr as isize);
        assert!(result <= 0);

        window
    }
}

#[derive(Debug)]
pub struct Window {
    pub hwnd: isize,
    pub display_scale: f32,
    //GDI related
    pub dc: *mut c_void,
    pub buffer: Vec<u32>,
    pub bitmap: BITMAPINFO,
    pub area: Rect,
    pub quit: bool,
    pub mouse_position: Rect,
    pub left_mouse: MouseButtonState,
    pub right_mouse: MouseButtonState,
    pub middle_mouse: MouseButtonState,
    pub mouse_4: MouseButtonState,
    pub mouse_5: MouseButtonState,
}

impl Window {
    ///Updates the width and height based on the display scale.
    pub fn rescale_window(&self) {
        let area = self.client_area();
        let (width, height) = if self.display_scale == 1.0 {
            (
                area.width as f32 / self.display_scale,
                area.height as f32 / self.display_scale,
            )
        } else {
            (
                area.width as f32 * self.display_scale,
                area.height as f32 * self.display_scale,
            )
        };

        unsafe {
            SetWindowPos(
                self.hwnd,
                0,
                area.x as i32,
                area.y as i32,
                width as i32,
                height as i32,
                SWP_FRAMECHANGED,
            )
        };
    }
    pub const fn display_scale(&self) -> f32 {
        self.display_scale
    }
    #[inline]
    pub fn client_area(&self) -> Rect {
        let mut rect = RECT::default();
        let _ = unsafe { GetClientRect(self.hwnd, &mut rect) };
        Rect::from_windows(rect)
    }
    #[inline(always)]
    pub const fn width(&self) -> usize {
        self.area.width
    }
    #[inline(always)]
    pub const fn height(&self) -> usize {
        self.area.height
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
    pub fn set_pos(&mut self, x: usize, y: usize, width: usize, height: usize, flags: u32) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                0,
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                flags,
            );
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
    pub fn event(&self) -> Option<Event> {
        if self.quit {
            return Some(Event::Quit);
        }

        unsafe {
            let mut msg = MSG::new();
            let result = PeekMessageA(&mut msg, self.hwnd, 0, 0, PM_REMOVE);
            translate_message(msg, result)
        }
    }
    pub fn event_blocking(&self) -> Option<Event> {
        if self.quit {
            return Some(Event::Quit);
        }

        unsafe {
            let mut msg = MSG::new();
            let result = GetMessageA(&mut msg, self.hwnd, 0, 0);
            translate_message(msg, result)
        }
    }
    pub fn vsync(&self) {
        unsafe { DwmFlush() };
    }
    //TODO: There is no support for depth.
    pub fn draw(&mut self) {
        // Not sure how to handle resets.
        // self.left_mouse.reset();
        // self.right_mouse.reset();
        // self.middle_mouse.reset();
        // self.mouse_4.reset();
        // self.mouse_5.reset();

        unsafe {
            StretchDIBits(
                self.dc,
                0,
                0,
                self.area.width as i32,
                self.area.height as i32,
                0,
                0,
                self.area.width as i32,
                self.area.height as i32,
                self.buffer.as_mut_ptr() as *const c_void,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }
    }
}

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

    pub const fn ex_style(mut self, flags: u32) -> Self {
        self.exstyle |= flags;
        self
    }

    pub const fn style(mut self, flags: u32) -> Self {
        self.style |= flags;
        self
    }
}

impl Default for WindowStyle {
    fn default() -> Self {
        Self::DEFAULT
    }
}

pub unsafe extern "system" fn wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    if msg == WM_CREATE {
        set_dark_theme(hwnd);
        return 0;
    }

    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window;
    if ptr.is_null() {
        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    //I'm not convinced this is the right way to do this.
    let window: &mut Window = &mut *ptr;

    let low = (lparam & 0xffff) as usize;
    let high = ((lparam >> 16) & 0xffff) as usize;

    match msg {
        //We can choose not to destroy the window, for example with a save prompt.
        WM_CLOSE => {
            assert!(DestroyWindow(hwnd) != 0);
            return 0;
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            window.quit = true;
            return 0;
        }
        //TODO: Could add a feature flag to skip this for no GDI use.
        //Do it in the UI library for now?
        WM_SIZE => {
            let (width, height) = (low, high);
            mini::info!("Resizing to width: {}, height: {}", width, height);
            window.buffer.clear();
            window.buffer.resize(width * height, 0);
            window.bitmap = BITMAPINFO::new(width as i32, height as i32);
            window.area = Rect::new(0, 0, width, height);

            return 0;
        }
        //https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged
        WM_DPICHANGED => {
            //The new display scale and DPI.
            let dpi = (wparam >> 16) & 0xffff;
            let scale = dpi as f32 / DEFAULT_DPI;

            //This is the recommended x, y, width and height.
            //The width and height is wrong so we ignore it.
            //X and Y seems right.
            let ptr = lparam as *mut RECT;
            assert!(!ptr.is_null());
            let rect = &(*ptr);

            let old = window.client_area();
            let original_width = old.width as f32 / window.display_scale;
            let original_height = old.height as f32 / window.display_scale;

            let (width, height) = if scale == 1.0 {
                (original_width, original_height)
            } else {
                (original_width * scale, original_height * scale)
            };

            mini::info!("Rescaling Window x: {}, y: {}, width: {}, height: {}, old_scale: {}, new_scale: {}", old.x, old.y, width.round(), height.round(), window.display_scale, scale);

            SetWindowPos(
                hwnd,
                0,
                rect.left,
                rect.top,
                width.round() as i32,
                height.round() as i32,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );

            window.display_scale = scale;
            return 0;
        }
        WM_MOUSEMOVE => {
            window.mouse_position = Rect::new(low, high, 1, 1);
            return 0;
        }
        WM_LBUTTONDOWN => {
            window.left_mouse.pressed(Rect::new(low, high, 1, 1));
            return 0;
        }
        WM_LBUTTONUP => {
            window.left_mouse.released(Rect::new(low, high, 1, 1));
            return 0;
        }
        WM_RBUTTONDOWN => {
            window.right_mouse.pressed(Rect::new(low, high, 1, 1));
            return 0;
        }
        WM_RBUTTONUP => {
            window.right_mouse.released(Rect::new(low, high, 1, 1));
            return 0;
        }
        WM_MBUTTONDOWN => {
            window.middle_mouse.pressed(Rect::new(low, high, 1, 1));
            return 0;
        }
        WM_MBUTTONUP => {
            window.middle_mouse.released(Rect::new(low, high, 1, 1));
            return 0;
        }
        WM_XBUTTONDOWN => {
            let button = ((wparam >> 16) & 0xffff) as usize;
            if button == 1 {
                window.mouse_4.pressed(Rect::new(low, high, 1, 1));
            } else if button == 2 {
                window.mouse_5.pressed(Rect::new(low, high, 1, 1));
            }
            return 0;
        }
        WM_XBUTTONUP => {
            let button = ((wparam >> 16) & 0xffff) as usize;
            if button == 1 {
                window.mouse_4.released(Rect::new(low, high, 1, 1));
            } else if button == 2 {
                window.mouse_5.released(Rect::new(low, high, 1, 1));
            }
            return 0;
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}
