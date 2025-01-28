use crate::*;

pub unsafe fn event(hwnd: Option<isize>) -> Option<Event> {
    let mut msg = MSG::new();
    let result = PeekMessageA(&mut msg, hwnd.unwrap_or_default(), 0, 0, PM_REMOVE);
    handle_msg(msg, result)
}

// fn event_blocking(hwnd: Option<isize>) -> Option<Event> {
//     let mut msg = MSG::new();
//     let result = unsafe { GetMessageA(&mut msg, hwnd.unwrap_or_default(), 0, 0) };
//     handle_msg(msg, result)
// }

//Event handling should probably happen in the UI library.
//It doesn't really make sense to return an event every time.
//There will be a context which will hold the state every frame.
//I think It would be nice to be able to use that context to store information.
//For example, on a mouse press, `ctx.left_mouse.pressed = true`
//Rather than return Some(Event::LeftMouseDown) and then having to set that later.
//It just doesn't make any sense.

//Note that some messages like WM_MOVE and WM_SIZE will not be included here.
//wndproc must be used for window related messages.
pub unsafe fn handle_msg(msg: MSG, result: i32) -> Option<Event> {
    //Mouse position.
    // let (x, y) = (msg.pt.x, msg.pt.y);

    let modifiers = modifiers();
    match result {
        -1 => {
            let last_error = GetLastError();
            panic!("Error with `GetMessageA`, error code: {}", last_error);
        }
        0 => None,
        _ => match msg.message {
            WM_MOUSEMOVE => {
                let x = msg.l_param & 0xFFFF;
                let y = msg.l_param >> 16 & 0xFFFF;
                Some(Event::MouseMoveInsideWindow(x as i32, y as i32))
            }
            WM_MOUSEWHEEL => {
                const WHEEL_DELTA: i16 = 120;
                let value = (msg.w_param >> 16) as i16;
                let delta = value as f32 / WHEEL_DELTA as f32;
                if delta >= 0.0 {
                    Some(Event::Input(Key::ScrollUp, modifiers))
                } else {
                    Some(Event::Input(Key::ScrollDown, modifiers))
                }
            }
            WM_LBUTTONDOWN => Some(Event::Input(Key::LeftMouseDown, modifiers)),
            WM_LBUTTONUP => Some(Event::Input(Key::LeftMouseUp, modifiers)),
            WM_LBUTTONDBLCLK => Some(Event::Input(Key::LeftMouseDoubleClick, modifiers)),
            WM_RBUTTONDOWN => Some(Event::Input(Key::RightMouseDown, modifiers)),
            WM_RBUTTONUP => Some(Event::Input(Key::RightMouseUp, modifiers)),
            WM_RBUTTONDBLCLK => Some(Event::Input(Key::RightMouseDoubleClick, modifiers)),
            WM_MBUTTONDOWN => Some(Event::Input(Key::MiddleMouseDown, modifiers)),
            WM_MBUTTONUP => Some(Event::Input(Key::MiddleMouseUp, modifiers)),
            WM_MBUTTONDBLCLK => Some(Event::Input(Key::MiddleMouseDoubleClick, modifiers)),
            WM_XBUTTONDOWN => {
                //https://www.autohotkey.com/docs/v1/KeyList.htm#mouse-advanced
                //XButton1	4th mouse button. Typically performs the same function as Browser_Back.
                //XButton2	5th mouse button. Typically performs the same function as Browser_Forward.
                let button = msg.w_param >> 16;
                if button == 1 {
                    Some(Event::Input(Key::Mouse4Down, modifiers))
                } else if button == 2 {
                    Some(Event::Input(Key::Mouse5Down, modifiers))
                } else {
                    unreachable!()
                }
            }
            WM_XBUTTONUP => {
                let button = msg.w_param >> 16;
                if button == 1 {
                    Some(Event::Input(Key::Mouse4Up, modifiers))
                } else if button == 2 {
                    Some(Event::Input(Key::Mouse5Up, modifiers))
                } else {
                    unreachable!()
                }
            }
            WM_XBUTTONDBLCLK => {
                let button = msg.w_param >> 16;
                if button == 1 {
                    Some(Event::Input(Key::Mouse4DoubleClick, modifiers))
                } else if button == 2 {
                    Some(Event::Input(Key::Mouse5DoubleClick, modifiers))
                } else {
                    unreachable!()
                }
            }
            //https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown
            WM_KEYDOWN => {
                let vk = msg.w_param as i32;
                let shift = modifiers.shift;

                match vk {
                    VK_UP => return Some(Event::Input(Key::Up, modifiers)),
                    VK_DOWN => return Some(Event::Input(Key::Down, modifiers)),
                    VK_LEFT => return Some(Event::Input(Key::Left, modifiers)),
                    VK_RIGHT => return Some(Event::Input(Key::Right, modifiers)),
                    VK_RETURN => return Some(Event::Input(Key::Enter, modifiers)),
                    VK_SPACE => return Some(Event::Input(Key::Char(' '), modifiers)),
                    VK_BACK => return Some(Event::Input(Key::Backspace, modifiers)),
                    VK_ESCAPE => return Some(Event::Input(Key::Escape, modifiers)),
                    VK_TAB => return Some(Event::Input(Key::Tab, modifiers)),
                    VK_LWIN => return Some(Event::Input(Key::LeftWindows, modifiers)),
                    VK_RWIN => return Some(Event::Input(Key::RightWindows, modifiers)),
                    VK_APPS => return Some(Event::Input(Key::Menu, modifiers)),
                    VK_SCROLL => return Some(Event::Input(Key::ScrollLock, modifiers)),
                    VK_PAUSE => return Some(Event::Input(Key::PauseBreak, modifiers)),
                    VK_INSERT => return Some(Event::Input(Key::Insert, modifiers)),
                    VK_HOME => return Some(Event::Input(Key::Home, modifiers)),
                    VK_END => return Some(Event::Input(Key::End, modifiers)),
                    VK_PRIOR => return Some(Event::Input(Key::PageUp, modifiers)),
                    VK_NEXT => return Some(Event::Input(Key::PageDown, modifiers)),
                    VK_DELETE => return Some(Event::Input(Key::Delete, modifiers)),
                    VK_SHIFT | VK_LSHIFT | VK_RSHIFT => {
                        return Some(Event::Input(Key::Shift, modifiers))
                    }
                    VK_CONTROL | VK_LCONTROL | VK_RCONTROL => {
                        return Some(Event::Input(Key::Control, modifiers))
                    }
                    //TODO: Does not work
                    // VK_SNAPSHOT => return Some(Event::Input(Key::PrintScreen, modifiers)),
                    //TODO: Alt does not work
                    // VK_MENU | VK_LMENU | VK_RMENU => {
                    //     return Some(Event::Input(Key::Alt, modifiers))
                    // }
                    VK_OEM_PLUS if shift => return Some(Event::Input(Key::Char('+'), modifiers)),
                    VK_OEM_MINUS if shift => return Some(Event::Input(Key::Char('_'), modifiers)),
                    VK_OEM_3 if shift => return Some(Event::Input(Key::Char('~'), modifiers)),
                    VK_OEM_4 if shift => return Some(Event::Input(Key::Char('{'), modifiers)),
                    VK_OEM_6 if shift => return Some(Event::Input(Key::Char('}'), modifiers)),
                    VK_OEM_5 if shift => return Some(Event::Input(Key::Char('|'), modifiers)),
                    VK_OEM_1 if shift => return Some(Event::Input(Key::Char(':'), modifiers)),
                    VK_OEM_7 if shift => return Some(Event::Input(Key::Char('"'), modifiers)),
                    VK_OEM_COMMA if shift => return Some(Event::Input(Key::Char('<'), modifiers)),
                    VK_OEM_PERIOD if shift => return Some(Event::Input(Key::Char('>'), modifiers)),
                    VK_OEM_2 if shift => return Some(Event::Input(Key::Char('?'), modifiers)),
                    VK_OEM_PLUS => return Some(Event::Input(Key::Char('='), modifiers)),
                    VK_OEM_MINUS => return Some(Event::Input(Key::Char('-'), modifiers)),
                    VK_OEM_3 => return Some(Event::Input(Key::Char('`'), modifiers)),
                    VK_OEM_4 => return Some(Event::Input(Key::Char('['), modifiers)),
                    VK_OEM_6 => return Some(Event::Input(Key::Char(']'), modifiers)),
                    VK_OEM_5 => return Some(Event::Input(Key::Char('\\'), modifiers)),
                    VK_OEM_1 => return Some(Event::Input(Key::Char(';'), modifiers)),
                    VK_OEM_7 => return Some(Event::Input(Key::Char('\''), modifiers)),
                    VK_OEM_COMMA => return Some(Event::Input(Key::Char(','), modifiers)),
                    VK_OEM_PERIOD => return Some(Event::Input(Key::Char('.'), modifiers)),
                    VK_OEM_2 => return Some(Event::Input(Key::Char('/'), modifiers)),
                    VK_F1..=VK_F24 => {
                        return Some(Event::Input(
                            Key::Function((vk - VK_F1 as i32 + 1) as u8),
                            modifiers,
                        ))
                    }
                    //(A-Z,modifiers)) (0-9,modifiers))
                    0x30..=0x39 | 0x41..=0x5A => {
                        let vk = vk as u8 as char;
                        if shift {
                            Some(Event::Input(
                                Key::Char(match vk {
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
                                }),
                                modifiers,
                            ))
                        } else {
                            //I think all alphabetical inputs are UPPERCASE.
                            Some(Event::Input(Key::Char(vk.to_ascii_lowercase()), modifiers))
                        }
                    }
                    _ => Some(Event::Input(Key::Unknown(vk as u16), modifiers)),
                }
            }
            _ => {
                // TODO: Is this dispatch garbage even needed?
                // TranslateMessage(addr_of_mut!(msg));
                // DispatchMessageA(addr_of_mut!(msg));
                wnd_proc(msg.hwnd, msg.message, msg.w_param, msg.l_param);
                None
            }
        },
    }
}
