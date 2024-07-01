use crate::*;
use crossbeam_queue::SegQueue;

#[derive(Debug)]
pub struct Window {
    pub hwnd: isize,
    pub screen_mouse_pos: (i32, i32),
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
    pub fn event(&self) -> Option<Event> {
        //Window procedure events take presidence here.
        if let Some(event) = event(Some(self.hwnd)) {
            self.queue.push(event)
        }

        self.queue.pop()
    }
}

//TODO: Remove
static mut TEST: isize = 0;

unsafe extern "system" fn wnd_proc(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    if msg == WM_CREATE {
        set_dark_mode(hwnd).unwrap();
        return 0;
    }

    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window;
    assert_eq!(ptr as isize, TEST);

    if ptr.is_null() {
        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    //I'm not convinced this is the right way to do this.
    let window: &mut Window = &mut *ptr;

    match msg {
        WM_DESTROY | WM_CLOSE => {
            // window.queue.push(Event::Quit);

            //Quit as fast as possible.
            QUIT = true;
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
            // window.queue.push(Event::Move);
            return 0;
        }
        WM_SIZE => {
            window.queue.push(Event::Resize);
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

pub unsafe fn create_window(
    title: &str,
    // x: Option<i32>,
    // y: Option<i32>,
    width: i32,
    height: i32,
) -> std::pin::Pin<Box<Window>> {
    const WINDOW_STYLE: u32 = 0;
    //Basically every window option that people use nowadays is completely pointless.
    const WINDOW_OPTIONS: u32 = WS_OVERLAPPEDWINDOW | WS_VISIBLE;

    if SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) as u32 == 0 {
        panic!("Only Windows 10 (1607) or later is supported.")
    };

    //Title must be null terminated.
    let title = std::ffi::CString::new(title).unwrap();

    let wnd_class = WNDCLASSA {
        wnd_proc: Some(wnd_proc),
        class_name: title.as_ptr() as *const u8,
        style: WINDOW_STYLE,
        background: 0,
        //Prevent cursor from changing when loading.
        cursor: LoadCursorW(null_mut(), IDC_ARROW) as isize,
        ..Default::default()
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
        WINDOW_OPTIONS,
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

    //Safety: This *should* be pinned.
    let window = Box::pin(Window {
        hwnd,
        screen_mouse_pos: (0, 0),
        queue: SegQueue::new(),
    });

    let addr = &*window as *const Window;
    let result = SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, addr as isize);
    assert!(result <= 0);
    TEST = addr as isize;

    window
}
