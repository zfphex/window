use crate::*;

// pub fn is_maximized(window: HWND) -> bool {
//     unsafe {
//         let mut placement: WINDOWPLACEMENT = mem::zeroed();
//         placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
//         GetWindowPlacement(window, &mut placement);
//         placement.showCmd == SW_MAXIMIZE
//     }
// }

///To get the window bounds excluding the drop shadow, use DwmGetWindowAttribute, specifying DWMWA_EXTENDED_FRAME_BOUNDS. Note that unlike the Window Rect, the DWM Extended Frame Bounds are not adjusted for DPI. Getting the extended frame bounds can only be done after the window has been shown at least once.
pub fn screen_area_no_shadow(hwnd: isize) -> RECT {
    todo!();
}

///WinRect coordiantes can be negative.
pub fn screen_area(hwnd: isize) -> RECT {
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect) };
    rect
}

///WinRect coordiantes *should* never be negative.
pub fn client_area(hwnd: isize) -> RECT {
    let mut rect = RECT::default();
    unsafe { GetClientRect(hwnd, &mut rect) };
    rect
}

pub fn desktop_area() -> RECT {
    unsafe { client_area(GetDesktopWindow()) }
}

// pub fn screen_to_client(hwnd: isize) -> WinRect {
//     // let mut rect = WinRect::default();
//     // unsafe { GetWindowRect(hwnd, &mut rect) };
//     // rect
// }

//YEP
pub static mut WINDOW_AREA: RECT = RECT::new(0, 0, 0, 0);

pub struct Window {
    pub hwnd: isize,
    pub context: *mut VOID,
}

impl Window {
    //REMOVE

    // pub fn get_long_ptr(&self) -> isize {
    //     unsafe { GetWindowLongPtrA(self.hwnd, GWLP_USERDATA) }
    // }
    // pub fn set_long_ptr(&self) {
    //     unsafe { GetWindowLongPtrA(std::mem::transmute(self), GWLP_USERDATA) };
    // }

    pub fn client_area(&self) -> RECT {
        client_area(self.hwnd)
    }

    pub fn screen_area(&self) -> RECT {
        screen_area(self.hwnd)
    }

    //TODO: Swap between fullscreen and windowed.
    //https://devblogs.microsoft.com/oldnewthing/20100412-00/?p=14353
    pub fn fullscreen(&self) {
        unsafe { ShowWindow(self.hwnd, SW_MAXIMIZE) };
    }
}

unsafe extern "system" fn wnd_proc(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    //TODO: Handle dragging.
    //https://github.com/rust-windowing/winit/blob/7bed5eecfdcbde16e5619fd137f0229e8e7e8ed4/src/platform_impl/windows/window.rs#L474C21-L474C21

    match msg {
        WM_DESTROY | WM_CLOSE => {
            QUIT = true;
            //TODO: Check if this closes the window faster.
            //I think windows cleans up the window when the application is closed.
            //So I don't really care.
            // DestroyWindow(hwnd);
            // PostQuitMessage(0);
            return 0;
        }
        WM_CREATE => {
            if !set_dark_mode(hwnd) {
                println!("Failed to set dark mode!");
            }
            return 0;
        }
        WM_ERASEBKGND => {
            return 1;
        }
        WM_PAINT => {
            //The BeginPaint function automatically validates the entire client area.
            return 0;
        }
        WM_MOVE => {
            return 0;
        }
        // WM_MOVE => {
        //     let x = (MSG.l_param as u32) & 0xffff;
        //     let y = ((MSG.l_param as u32) >> 16) & 0xffff;

        //     let width = WINDOW_AREA.width();
        //     let height = WINDOW_AREA.height();

        //     WINDOW_AREA.left = x as i32;
        //     WINDOW_AREA.top = y as i32;
        //     WINDOW_AREA.right = x as i32 + width;
        //     WINDOW_AREA.bottom = y as i32 + height;

        //     return 0;
        // }
        //https://billthefarmer.github.io/blog/post/handling-resizing-in-windows/
        //https://github.com/not-fl3/miniquad/blob/f6780f19d3592077019872850d00e5eb9e92a22d/src/native/windows.rs#L214
        // WM_SIZE => {
        //     //When resizing the window horizontally the height changes.
        //     //This should not be possible?

        //     //TODO: These must be totally wrong.
        //     // let width = (MSG.l_param as u32) & 0xffff;
        //     // let height = ((MSG.l_param as u32) >> 16) & 0xffff;

        //     let mut rect = WinRect::default();
        //     GetClientRect(hwnd, &mut rect);

        //     let mut top_left = Point {
        //         x: rect.left,
        //         y: rect.top,
        //     };
        //     ClientToScreen(hwnd, &mut top_left);

        //     let mut bottom_right = Point {
        //         x: rect.right,
        //         y: rect.bottom,
        //     };
        //     ClientToScreen(hwnd, &mut bottom_right);

        //     SetRect(
        //         &mut rect,
        //         top_left.x,
        //         top_left.y,
        //         bottom_right.x,
        //         bottom_right.y,
        //     );

        //     WINDOW_AREA = rect;

        //     return 0;
        // }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

pub fn create_window(title: &str, x: i32, y: i32, width: i32, height: i32) -> Window {
    //CS_HREDRAW AND CS_VREDRAW ARE FOR WM_PAINT I THINK?
    //We don't need them.
    //const WINDOW_STYLE: u32 = CS_HREDRAW | CS_VREDRAW;
    //I don't think CS_OWNDC is needed either.
    //https://devblogs.microsoft.com/oldnewthing/20060601-06/?p=31003
    const WINDOW_STYLE: u32 = 0;
    const WINDOW_OPTIONS: u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE;

    unsafe {
        //Title must be null terminated.
        let title = std::ffi::CString::new(title).unwrap();
        let wnd_class = WNDCLASSA {
            // wnd_proc: Some(DefWindowProcA),
            wnd_proc: Some(wnd_proc),
            class_name: title.as_ptr() as *const u8,
            style: WINDOW_STYLE,
            background: 0,
            //Prevent cursor from changing when loading.
            cursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW) as isize,
            ..Default::default()
        };

        let _ = RegisterClassA(&wnd_class);

        // WINDOW_AREA.top = x;
        // WINDOW_AREA.left = y;
        // WINDOW_AREA.right = width;
        // WINDOW_AREA.bottom = height;

        //Imagine that the users wants a window that is 800x600.
        //`CreateWindow` takes in screen coordinates instead of client coordiantes.
        //Which means that it will set the window size including the title bar and borders etc.
        //We must convert the requested client coordinates to screen coordinates.

        //TODO: What is this value at different DPI's?
        const WINDOW_PADDING_96_DPI: i32 = 7;

        let rect = RECT {
            left: x - WINDOW_PADDING_96_DPI,
            top: y,
            right: x + width,
            bottom: y + height,
        };
        // let _ = AdjustWindowRectEx(&mut rect, WINDOW_OPTIONS, 0, 0);

        // let result = AdjustWindowRectEx(&mut rect, 0, 0, 0);
        // assert_eq!(rect.width(), width);
        // assert_eq!(rect.height(), height);
        // if result == 0 {
        //     let last_error = GetLastError();
        //     panic!(
        //         "Error with `AdjustWindowRectEx`, error code: {}",
        //         last_error
        //     );
        // }

        // let h_instance = get_hinstance();
        let hwnd = CreateWindowExA(
            0,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            WINDOW_OPTIONS,
            //Previously I was using CW_USEDEFAULT for x and y.
            //This is not equal to 0, 0 but it is a good starting position.
            rect.left,
            rect.top,
            rect.width(),
            rect.height(),
            0,
            0,
            0, // h_instance,
            std::ptr::null(),
        );

        assert_ne!(hwnd, 0);

        let context = GetDC(hwnd);

        Window { hwnd, context }
    }
}
