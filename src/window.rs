use crate::*;

// pub fn is_maximized(window: HWND) -> bool {
//     unsafe {
//         let mut placement: WINDOWPLACEMENT = mem::zeroed();
//         placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
//         GetWindowPlacement(window, &mut placement);
//         placement.showCmd == SW_MAXIMIZE
//     }
// }

//YEP
static mut WINDOW_AREA: WinRect = WinRect::new(0, 0, 0, 0);

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
    pub fn area(&self) -> WinRect {
        // let mut rect = WinRect::default();
        //GetWindowRect is virtualized for DPI.
        // unsafe { GetWindowRect(self.hwnd, &mut rect) };
        // rect
        unsafe { WINDOW_AREA.clone() }
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

unsafe extern "system" fn test_proc(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    // let user_data = GetWindowLongA(hwnd, GWLP_USERDATA);
    // if user_data == 0 {
    //     return DefWindowProcA(hwnd, msg, wparam, lparam);
    // }

    // let _window: &mut Window = std::mem::transmute(user_data as *mut i32);

    //TODO: Handle dragging.
    //https://github.com/rust-windowing/winit/blob/7bed5eecfdcbde16e5619fd137f0229e8e7e8ed4/src/platform_impl/windows/window.rs#L474C21-L474C21

    match msg {
        WM_DESTROY | WM_CLOSE => {
            QUIT = true;
            // PostQuitMessage(0);
            return 0;
        }
        // WM_CLOSE => {
        // DestroyWindow(hwnd);
        // }
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
            let x = (MSG.l_param as u32) & 0xffff;
            let y = ((MSG.l_param as u32) >> 16) & 0xffff;

            let width = WINDOW_AREA.width();
            let height = WINDOW_AREA.height();

            WINDOW_AREA.left = x as i32;
            WINDOW_AREA.top = y as i32;
            WINDOW_AREA.right = x as i32 + width;
            WINDOW_AREA.bottom = y as i32 + height;

            return 0;
        }
        WM_SIZE => {
            //If the size never updates it can't crash!

            //When resizing the window horizontally the height changes.
            //This should not be possible?

            // let width = (MSG.l_param as u32) & 0xffff;
            // let height = ((MSG.l_param as u32) >> 16) & 0xffff;
            // println!("width: {}, height: {}", width, height);
            // let _ = adjust_window(width as i32, height as i32);

            return 0;
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

const OPTIONS: u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
//Redraw the window on veritcal and horizontal resize.
//https://devblogs.microsoft.com/oldnewthing/20060601-06/?p=31003
const STYLE: u32 = CS_HREDRAW | CS_VREDRAW;

//TODO: https://devblogs.microsoft.com/oldnewthing/20100412-00/?p=14353
pub fn create_window(title: &str, width: i32, height: i32) -> Window {
    unsafe {
        //Title must be null terminated.
        let title = std::ffi::CString::new(title).unwrap();
        let wnd_class = WNDCLASSA {
            // wnd_proc: Some(DefWindowProcA),
            wnd_proc: Some(test_proc),
            class_name: title.as_ptr() as *const u8,
            style: STYLE,
            background: 0,
            //Prevent cursor from changing when loading.
            cursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW) as isize,
            ..Default::default()
        };

        let _ = RegisterClassA(&wnd_class);

        // let mut rect = WinRect {
        //     top: 0,
        //     left: 0,
        //     right: width,
        //     bottom: height,
        // };
        // let result = AdjustWindowRectEx(&mut rect as *mut WinRect, OPTIONS, 0, 0);
        WINDOW_AREA.right = width;
        WINDOW_AREA.bottom = height;
        let result = AdjustWindowRectEx(addr_of_mut!(WINDOW_AREA), OPTIONS, 0, 0);

        if result == 0 {
            let last_error = GetLastError();
            panic!(
                "Error with `AdjustWindowRectEx`, error code: {}",
                last_error
            );
        }

        // let h_instance = get_hinstance();
        let hwnd = CreateWindowExA(
            0,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            OPTIONS,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            //NOTE: Width and height include the border.
            // rect.width(),
            // rect.heigth(),
            WINDOW_AREA.width(),
            WINDOW_AREA.height(),
            0,
            0,
            0,
            // h_instance,
            std::ptr::null(),
        );

        assert_ne!(hwnd, 0);

        Window { hwnd }
    }
}
