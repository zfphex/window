use crate::*;

// pub fn is_maximized(window: HWND) -> bool {
//     unsafe {
//         let mut placement: WINDOWPLACEMENT = mem::zeroed();
//         placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
//         GetWindowPlacement(window, &mut placement);
//         placement.showCmd == SW_MAXIMIZE
//     }
// }

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
}

//TODO: https://github.com/makepad/makepad/blob/master/libs/windows-core/src/hresult.rs#L29
impl Window {
    pub fn get_long_ptr(&self) -> isize {
        unsafe { GetWindowLongPtrA(self.hwnd, GWLP_USERDATA) }
    }

    pub fn set_long_ptr(&self) {
        unsafe { GetWindowLongPtrA(std::mem::transmute(self), GWLP_USERDATA) };
    }

    //TODO: This is returning inaccurate results.
    //WinRect {
    //    top: 674,
    //    left: 109,
    //    right: 1313,
    //    bottom: 725,
    //}
    //Should have been equivalent to:
    //WinRect {
    //    top: 26,
    //    left: 26,
    //    right: 665,
    //    bottom: 642,
    //}
    pub fn area(&self) -> &RECT {
        // let mut rect = WinRect::default();
        //GetWindowRect is virtualized for DPI.
        // unsafe { GetWindowRect(self.hwnd, &mut rect) };
        // rect
        unsafe { &WINDOW_AREA }
    }

    //TODO: Remove?
    pub fn outer_position(&self) -> Point {
        let area = self.area();
        Point {
            x: area.left,
            y: area.top,
        }
    }

    pub fn inner_position(&self) -> Point {
        let mut point = Point::default();
        let result = unsafe { ClientToScreen(self.hwnd, &mut point) };
        assert_ne!(result, 0);
        point
    }

    pub fn fullscreen(&self) {
        unsafe { ShowWindow(self.hwnd, SW_MAXIMIZE) };
    }
}

//TODO: https://devblogs.microsoft.com/oldnewthing/20100412-00/?p=14353
pub fn create_window(title: &str, x: i32, y: i32, width: i32, height: i32) -> Window {
    const WINDOW_OPTIONS: u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
    //Redraw the window on veritcal and horizontal resize.
    //https://devblogs.microsoft.com/oldnewthing/20060601-06/?p=31003
    const WINDOW_STYLE: u32 = CS_HREDRAW | CS_VREDRAW;

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

        let mut rect = RECT {
            left: -8,
            top: 0,
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

        let screen = screen_area(hwnd);
        let client = client_area(hwnd);
        dbg!(screen, client);

        Window { hwnd }
    }
}
