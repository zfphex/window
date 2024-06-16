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
    pub screen_mouse_pos: (i32, i32),
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

    pub fn event(&mut self, hwnd: Option<isize>) -> Option<Event> {
        unsafe {
            if QUIT {
                return Some(Event::Quit);
            }

            //Does this actually do anything???
            let hwnd = hwnd.unwrap_or_default();

            //Note that some messages like WM_MOVE and WM_SIZE will not be included here.
            //wndproc must be used for window related messages.
            let result = PeekMessageA(addr_of_mut!(MSG), hwnd, 0, 0, PM_REMOVE);

            //Mouse position.
            let mp = (MSG.pt.x, MSG.pt.y);
            if self.screen_mouse_pos != mp {
                self.screen_mouse_pos = mp;
                //Event::ScreenMouseMove?
            }

            match result {
                0 => None,
                _ => match MSG.message {
                    WM_MOVE => Some(Event::Move),
                    WM_MOUSEMOVE => {
                        let x = MSG.l_param & 0xFFFF;
                        let y = MSG.l_param >> 16 & 0xFFFF;
                        Some(Event::Mouse(x as i32, y as i32))
                    }
                    WM_MOUSEWHEEL => {
                        const WHEEL_DELTA: i16 = 120;
                        let value = (MSG.w_param >> 16) as i16;
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
                        let button = MSG.w_param >> 16;
                        if button == 1 {
                            Some(Event::Mouse4Down)
                        } else if button == 2 {
                            Some(Event::Mouse5Down)
                        } else {
                            unreachable!()
                        }
                    }
                    WM_XBUTTONUP => {
                        let button = MSG.w_param >> 16;
                        if button == 1 {
                            Some(Event::Mouse4Up)
                        } else if button == 2 {
                            Some(Event::Mouse5Up)
                        } else {
                            unreachable!()
                        }
                    }
                    WM_XBUTTONDBLCLK => {
                        let button = MSG.w_param >> 16;
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
                        let vk = MSG.w_param as i32;
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

                            //TODO: Tilde is kind of an odd ball.
                            //Might need to handle this one better.
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
                        TranslateMessage(addr_of_mut!(MSG));
                        DispatchMessageA(addr_of_mut!(MSG));
                        None
                    }
                },
            }
        }
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

        Window {
            hwnd,
            context,
            screen_mouse_pos: (0, 0),
        }
    }
}
