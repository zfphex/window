use crate::*;

#[link(name = "user32")]
extern "system" {
    pub fn ToUnicode(
        wVirtKey: u32,
        wScanCode: u32,
        lpKeyState: *const u8,
        pwszBuff: *mut u16,
        cchBuff: i32,
        wFlags: u32,
    ) -> i32;
    pub fn ToUnicodeEx(
        wVirtKey: u32,
        wScanCode: u32,
        lpKeyState: *const u8,
        pwszBuff: *mut u16,
        cchBuff: i32,
        wFlags: u32,
        dwhkl: *mut c_void,
    ) -> i32;
    pub fn GetKeyboardState(lpKeyState: *mut u8) -> i32;
    pub fn GetKeyboardLayout(idThread: u32) -> *mut c_void;
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    None,
    LeftControl,
    LeftShift,
    LeftAlt,
    RightControl,
    RightShift,
    RightAlt,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Quit,
    //(0, 0) is top left of window.
    MouseMoveInsideWindow(i32, i32),
    //Global mouse move. Should not show up using `Window`.
    MouseMoveGlobal(i32, i32),
    Move,
    Input(Key, Modifiers),
}

#[derive(Debug, PartialEq)]
pub enum Key {
    Char(char),
    Function(u8),
    Enter,
    Backspace,
    Escape,
    Control,
    Shift,
    Alt,
    Tab,

    Up,
    Down,
    Left,
    Right,

    LeftMouseDown,
    LeftMouseUp,
    LeftMouseDoubleClick,

    MiddleMouseDown,
    MiddleMouseUp,
    MiddleMouseDoubleClick,

    RightMouseDown,
    RightMouseUp,
    RightMouseDoubleClick,

    Mouse4Down,
    Mouse4Up,
    Mouse4DoubleClick,

    Mouse5Down,
    Mouse5Up,
    Mouse5DoubleClick,

    ScrollUp,
    ScrollDown,

    Unknown(u16),
    LeftWindows,
    RightWindows,
    Menu,
    ScrollLock,
    PauseBreak,
    Insert,
    Home,
    Delete,
    End,
    PageUp,
    PageDown,
    PrintScreen,
}

impl Key {
    pub const fn into(self, modifiers: Modifiers) -> Option<Event> {
        Some(Event::Input(self, modifiers))
    }
}

#[derive(Debug, PartialEq)]
pub struct Modifiers {
    pub control: bool,
    pub shift: bool,
    pub alt: bool,
    pub win: bool,
}

//https://github.com/makepad/makepad/blob/69bef6bab686284e1e3ab83ee803f29c5c9f40e5/platform/src/os/windows/win32_window.rs#L765
pub fn modifiers() -> Modifiers {
    unsafe {
        Modifiers {
            control: GetKeyState(VK_CONTROL) & 0x80 > 0,
            shift: GetKeyState(VK_SHIFT) & 0x80 > 0,
            alt: GetKeyState(VK_MENU) & 0x80 > 0,
            win: GetKeyState(VK_LWIN) & 0x80 > 0 || GetKeyState(VK_RWIN) & 0x80 > 0,
        }
    }
}

//Event handling should probably happen in the UI library.
//It doesn't really make sense to return an event every time.
//There will be a context which will hold the state every frame.
//I think It would be nice to be able to use that context to store information.
//For example, on a mouse press, `ctx.left_mouse.pressed = true`
//Rather than return Some(Event::LeftMouseDown) and then having to set that later.
//It just doesn't make any sense.

//Note that some messages like WM_MOVE and WM_SIZE will not be included here.
//wndproc must be used for window related messages.
pub unsafe fn translate_message(msg: MSG, message_result: i32) -> Option<Event> {
    let (mouse_x, mouse_y) = (msg.pt.x, msg.pt.y);
    let modifiers = modifiers();

    if message_result == -1 {
        let last_error = GetLastError();
        panic!("Error with `GetMessageA`, error code: {}", last_error);
    }

    if message_result == 0 {
        return None;
    }

    match msg.message {
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

            return Some(match vk {
                // TODO: Doesn't work.
                // VK_SNAPSHOT => Event::Input(Key::PrintScreen, modifiers),
                VK_ESCAPE => Event::Input(Key::Escape, modifiers),
                VK_TAB => Event::Input(Key::Tab, modifiers),
                VK_UP => Event::Input(Key::Up, modifiers),
                VK_DOWN => Event::Input(Key::Down, modifiers),
                VK_LEFT => Event::Input(Key::Left, modifiers),
                VK_RIGHT => Event::Input(Key::Right, modifiers),
                VK_LWIN => Event::Input(Key::LeftWindows, modifiers),
                VK_RWIN => Event::Input(Key::RightWindows, modifiers),
                VK_APPS => Event::Input(Key::Menu, modifiers),
                VK_SCROLL => Event::Input(Key::ScrollLock, modifiers),
                VK_PAUSE => Event::Input(Key::PauseBreak, modifiers),
                VK_INSERT => Event::Input(Key::Insert, modifiers),
                VK_HOME => Event::Input(Key::Home, modifiers),
                VK_END => Event::Input(Key::End, modifiers),
                VK_PRIOR => Event::Input(Key::PageUp, modifiers),
                VK_NEXT => Event::Input(Key::PageDown, modifiers),
                VK_DELETE => Event::Input(Key::Delete, modifiers),
                VK_SHIFT | VK_LSHIFT | VK_RSHIFT => Event::Input(Key::Shift, modifiers),
                VK_CONTROL | VK_LCONTROL | VK_RCONTROL => Event::Input(Key::Control, modifiers),
                VK_F1..=VK_F24 => {
                    Event::Input(Key::Function((vk - VK_F1 as i32 + 1) as u8), modifiers)
                }
                VK_OEM_PLUS if shift => Event::Input(Key::Char('+'), modifiers),
                VK_OEM_MINUS if shift => Event::Input(Key::Char('_'), modifiers),
                VK_OEM_3 if shift => Event::Input(Key::Char('~'), modifiers),
                VK_OEM_4 if shift => Event::Input(Key::Char('{'), modifiers),
                VK_OEM_6 if shift => Event::Input(Key::Char('}'), modifiers),
                VK_OEM_5 if shift => Event::Input(Key::Char('|'), modifiers),
                VK_OEM_1 if shift => Event::Input(Key::Char(':'), modifiers),
                VK_OEM_7 if shift => Event::Input(Key::Char('"'), modifiers),
                VK_OEM_COMMA if shift => Event::Input(Key::Char('<'), modifiers),
                VK_OEM_PERIOD if shift => Event::Input(Key::Char('>'), modifiers),
                VK_OEM_2 if shift => Event::Input(Key::Char('?'), modifiers),
                VK_OEM_PLUS => Event::Input(Key::Char('='), modifiers),
                VK_OEM_MINUS => Event::Input(Key::Char('-'), modifiers),
                VK_OEM_3 => Event::Input(Key::Char('`'), modifiers),
                VK_OEM_4 => Event::Input(Key::Char('['), modifiers),

                VK_OEM_6 => Event::Input(Key::Char(']'), modifiers),
                VK_OEM_5 => Event::Input(Key::Char('\\'), modifiers),
                VK_OEM_1 => Event::Input(Key::Char(';'), modifiers),
                VK_OEM_7 => Event::Input(Key::Char('\''), modifiers),
                VK_OEM_COMMA => Event::Input(Key::Char(','), modifiers),
                VK_OEM_PERIOD => Event::Input(Key::Char('.'), modifiers),
                VK_OEM_2 => Event::Input(Key::Char('/'), modifiers),
                //(A-Z,modifiers) (0-9,modifiers)
                0x30..=0x39 | 0x41..=0x5A => {
                    let vk = vk as u8 as char;
                    if shift {
                        Event::Input(
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
                        )
                    } else {
                        //I think all alphabetical inputs are UPPERCASE.
                        Event::Input(Key::Char(vk.to_ascii_lowercase()), modifiers)
                    }
                }
                _ => Event::Input(Key::Unknown(vk as u16), modifiers),
            });
        }
        _ => {
            wnd_proc(msg.hwnd, msg.message, msg.w_param, msg.l_param);
            None
        }
    }
}
