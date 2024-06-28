use crate::*;
use crossbeam_queue::SegQueue;

#[derive(Debug)]
pub struct Window {
    pub hwnd: isize,
    pub screen_mouse_pos: (i32, i32),
    pub queue: SegQueue<Event>,
    pub init: bool,
}

impl Window {
    pub fn init(&mut self) {
        unsafe {
            SetWindowLongPtrW(self.hwnd, GWLP_USERDATA, self as *const _ as isize);
            self.init = true;
        }
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
        debug_assert!(self.init);

        unsafe {
            let mut msg = MSG::default();
            let result = PeekMessageA(addr_of_mut!(msg), self.hwnd, 0, 0, PM_REMOVE);

            let event = match result {
                0 => None,
                _ => match msg.message {
                    //This is not working :/
                    WM_GETDPISCALEDSIZE => Some(Event::Dpi(msg.w_param)),
                    WM_MOVE => Some(Event::Move),
                    WM_MOUSEMOVE => {
                        let x = msg.l_param & 0xFFFF;
                        let y = msg.l_param >> 16 & 0xFFFF;
                        Some(Event::Mouse(x as i32, y as i32))
                    }
                    WM_MOUSEWHEEL => {
                        const WHEEL_DELTA: i16 = 120;
                        let value = (msg.w_param >> 16) as i16;
                        let delta = value as f32 / WHEEL_DELTA as f32;
                        if delta >= 0.0 {
                            Some(Event::ScrollUp)
                        } else {
                            Some(Event::ScrollDown)
                        }
                    }
                    WM_LBUTTONDOWN => Some(Event::LeftMouseDown),
                    WM_LBUTTONUP => Some(Event::LeftMouseUp),
                    WM_LBUTTONDBLCLK => Some(Event::LeftMouseDoubleClick),
                    WM_RBUTTONDOWN => Some(Event::RightMouseDown),
                    WM_RBUTTONUP => Some(Event::RightMouseUp),
                    WM_RBUTTONDBLCLK => Some(Event::RightMouseDoubleClick),
                    WM_MBUTTONDOWN => Some(Event::MiddleMouseDown),
                    WM_MBUTTONUP => Some(Event::MiddleMouseUp),
                    WM_MBUTTONDBLCLK => Some(Event::MiddleMouseDoubleClick),
                    WM_XBUTTONDOWN => {
                        //https://www.autohotkey.com/docs/v1/KeyList.htm#mouse-advanced
                        //XButton1	4th mouse button. Typically performs the same function as Browser_Back.
                        //XButton2	5th mouse button. Typically performs the same function as Browser_Forward.
                        let button = msg.w_param >> 16;
                        if button == 1 {
                            Some(Event::Mouse4Down)
                        } else if button == 2 {
                            Some(Event::Mouse5Down)
                        } else {
                            unreachable!()
                        }
                    }
                    WM_XBUTTONUP => {
                        let button = msg.w_param >> 16;
                        if button == 1 {
                            Some(Event::Mouse4Up)
                        } else if button == 2 {
                            Some(Event::Mouse5Up)
                        } else {
                            unreachable!()
                        }
                    }
                    WM_XBUTTONDBLCLK => {
                        let button = msg.w_param >> 16;
                        if button == 1 {
                            Some(Event::Mouse4DoubleClick)
                        } else if button == 2 {
                            Some(Event::Mouse5DoubleClick)
                        } else {
                            unreachable!()
                        }
                    }
                    //https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown
                    WM_KEYDOWN => {
                        let vk = msg.w_param as i32;
                        let modifiers = modifiers();
                        let shift = modifiers.shift;

                        match vk {
                            VK_UP => return Some(Event::Up),
                            VK_DOWN => return Some(Event::Down),
                            VK_LEFT => return Some(Event::Left),
                            VK_RIGHT => return Some(Event::Right),
                            VK_RETURN => return Some(Event::Enter),
                            VK_SPACE => return Some(Event::Char(' ')),
                            VK_BACK => return Some(Event::Backspace),
                            VK_ESCAPE => return Some(Event::Escape),
                            VK_TAB => return Some(Event::Tab),
                            VK_SHIFT | VK_LSHIFT | VK_RSHIFT => return Some(Event::Shift),
                            VK_CONTROL | VK_LCONTROL | VK_RCONTROL => return Some(Event::Control),
                            VK_MENU | VK_LMENU | VK_RMENU => return Some(Event::Alt),
                            VK_OEM_PLUS if shift => return Some(Event::Char('+')),
                            VK_OEM_MINUS if shift => return Some(Event::Char('_')),
                            VK_OEM_3 if shift => return Some(Event::Char('~')),
                            VK_OEM_4 if shift => return Some(Event::Char('{')),
                            VK_OEM_6 if shift => return Some(Event::Char('}')),
                            VK_OEM_5 if shift => return Some(Event::Char('|')),
                            VK_OEM_1 if shift => return Some(Event::Char(':')),
                            VK_OEM_7 if shift => return Some(Event::Char('"')),
                            VK_OEM_COMMA if shift => return Some(Event::Char('<')),
                            VK_OEM_PERIOD if shift => return Some(Event::Char('>')),
                            VK_OEM_2 if shift => return Some(Event::Char('?')),
                            VK_OEM_PLUS => return Some(Event::Char('=')),
                            VK_OEM_MINUS => return Some(Event::Char('-')),
                            VK_OEM_3 => return Some(Event::Char('`')),
                            VK_OEM_4 => return Some(Event::Char('[')),
                            VK_OEM_6 => return Some(Event::Char(']')),
                            VK_OEM_5 => return Some(Event::Char('\\')),
                            VK_OEM_1 => return Some(Event::Char(';')),
                            VK_OEM_7 => return Some(Event::Char('\'')),
                            VK_OEM_COMMA => return Some(Event::Char(',')),
                            VK_OEM_PERIOD => return Some(Event::Char('.')),
                            VK_OEM_2 => return Some(Event::Char('/')),
                            VK_F1..=VK_F24 => {
                                return Some(Event::Function((vk - VK_F1 as i32 + 1) as u8))
                            }
                            //(A-Z) (0-9)
                            0x30..=0x39 | 0x41..=0x5A => {
                                let vk = vk as u8 as char;
                                if shift {
                                    Some(Event::Char(match vk {
                                        '1' => '!',
                                        '2' => '@',
                                        '3' => '#',
                                        '4' => '$',
                                        '5' => '%',
                                        '6' => '^',
                                        '7' => '&',
                                        '8' => '*',
                                        '9' => '(',
                                        '0' => ')',
                                        _ => vk,
                                    }))
                                } else {
                                    //I think all alphabetical inputs are UPPERCASE.
                                    Some(Event::Char(vk.to_ascii_lowercase()))
                                }
                            }
                            _ => Some(Event::Unknown(vk as u16)),
                        }
                    }
                    _ => {
                        TranslateMessage(addr_of_mut!(msg));
                        DispatchMessageA(addr_of_mut!(msg));
                        None
                    }
                },
            };

            //Window procedure events take presidence here.
            if let Some(event) = event {
                self.queue.push(event)
            }

            self.queue.pop()
        }
    }
}

unsafe extern "system" fn wnd_proc(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    let window = GetWindowLongPtrW(hwnd, GWLP_USERDATA);

    if window == 0 {
        match msg {
            WM_CREATE => {
                if !set_dark_mode(hwnd) {
                    println!("Failed to set dark mode!");
                }
                return 0;
            }
            _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
        }
    };

    let window = &mut (*(window as *mut Window));

    match msg {
        WM_DESTROY | WM_CLOSE => {
            window.queue.push(Event::Quit);
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
) -> Window {
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

    Window {
        hwnd,
        screen_mouse_pos: (0, 0),
        queue: SegQueue::new(),
        init: false,
    }
}
