use crate::*;

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
    ///Mouse movement inside the window. (0, 0) is top left of window.
    MouseMove(i32, i32),
    Input(Key, Modifiers),
}

#[derive(Debug, PartialEq)]
pub enum Key {
    Char(char),
    Function(u8),
    Enter,
    Space,
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
    Unknown(u16),
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

#[track_caller]
fn handle_mouse_button(button: usize, m4: Key, m5: Key) -> Key {
    match button {
        1 => m4,
        2 => m5,
        _ => unreachable!(),
    }
}

pub fn translate_message(msg: MSG, message_result: i32) -> Option<Event> {
    let (mouse_x, mouse_y) = (msg.pt.x, msg.pt.y);
    let modifiers = modifiers();

    if message_result == 0 {
        return None;
    } else if message_result == -1 {
        let last_error = unsafe { GetLastError() };
        panic!("Error with `GetMessageA`, error code: {}", last_error);
    }

    if msg.message == WM_MOUSEMOVE {
        let x = msg.l_param & 0xFFFF;
        let y = msg.l_param >> 16 & 0xFFFF;
        return Some(Event::MouseMove(x as i32, y as i32));
    }

    let key = match msg.message {
        WM_MOUSEWHEEL => {
            const WHEEL_DELTA: i16 = 120;
            let value = (msg.w_param >> 16) as i16;
            let delta = value as f32 / WHEEL_DELTA as f32;
            if delta >= 0.0 {
                Key::ScrollUp
            } else {
                Key::ScrollDown
            }
        }
        WM_LBUTTONDOWN => Key::LeftMouseDown,
        WM_LBUTTONUP => Key::LeftMouseUp,
        WM_LBUTTONDBLCLK => Key::LeftMouseDoubleClick,
        WM_RBUTTONDOWN => Key::RightMouseDown,
        WM_RBUTTONUP => Key::RightMouseUp,
        WM_RBUTTONDBLCLK => Key::RightMouseDoubleClick,
        WM_MBUTTONDOWN => Key::MiddleMouseDown,
        WM_MBUTTONUP => Key::MiddleMouseUp,
        WM_MBUTTONDBLCLK => Key::MiddleMouseDoubleClick,
        WM_XBUTTONDOWN => handle_mouse_button(msg.w_param >> 16, Key::Mouse4Down, Key::Mouse5Down),
        WM_XBUTTONUP => handle_mouse_button(msg.w_param >> 16, Key::Mouse4Up, Key::Mouse5Up),
        WM_XBUTTONDBLCLK => handle_mouse_button(
            msg.w_param >> 16,
            Key::Mouse4DoubleClick,
            Key::Mouse5DoubleClick,
        ),
        //https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown
        WM_KEYDOWN => {
            let vk = msg.w_param as i32;
            let shift = modifiers.shift;

            match vk {
                VK_SPACE => Key::Space,
                VK_ESCAPE => Key::Escape,
                VK_TAB => Key::Tab,
                VK_UP => Key::Up,
                VK_DOWN => Key::Down,
                VK_LEFT => Key::Left,
                VK_RIGHT => Key::Right,
                VK_LWIN => Key::LeftWindows,
                VK_RWIN => Key::RightWindows,
                VK_APPS => Key::Menu,
                VK_SCROLL => Key::ScrollLock,
                VK_PAUSE => Key::PauseBreak,
                VK_INSERT => Key::Insert,
                VK_HOME => Key::Home,
                VK_END => Key::End,
                VK_PRIOR => Key::PageUp,
                VK_NEXT => Key::PageDown,
                VK_DELETE => Key::Delete,
                //Some keyboards don't report left and right control/shift independently.
                //So there's no way to specifiy which one.
                VK_SHIFT | VK_LSHIFT | VK_RSHIFT => Key::Shift,
                VK_CONTROL | VK_LCONTROL | VK_RCONTROL => Key::Control,
                VK_F1..=VK_F24 => Key::Function((vk - VK_F1 as i32 + 1) as u8),
                VK_OEM_PLUS if shift => Key::Char('+'),
                VK_OEM_PLUS => Key::Char('='),
                VK_OEM_MINUS if shift => Key::Char('_'),
                VK_OEM_MINUS => Key::Char('-'),
                VK_OEM_1 if shift => Key::Char(':'),
                VK_OEM_1 => Key::Char(';'),
                VK_OEM_2 if shift => Key::Char('?'),
                VK_OEM_2 => Key::Char('/'),
                VK_OEM_3 if shift => Key::Char('~'),
                VK_OEM_3 => Key::Char('`'),
                VK_OEM_4 if shift => Key::Char('{'),
                VK_OEM_4 => Key::Char('['),
                VK_OEM_5 if shift => Key::Char('|'),
                VK_OEM_5 => Key::Char('\\'),
                VK_OEM_6 if shift => Key::Char('}'),
                VK_OEM_6 => Key::Char(']'),
                VK_OEM_7 if shift => Key::Char('"'),
                VK_OEM_7 => Key::Char('\''),
                VK_OEM_COMMA if shift => Key::Char('<'),
                VK_OEM_COMMA => Key::Char(','),
                VK_OEM_PERIOD if shift => Key::Char('>'),
                VK_OEM_PERIOD => Key::Char('.'),
                //(A-Z,modifiers) (0-9,modifiers)
                0x30..=0x39 | 0x41..=0x5A => {
                    let vk = vk as u8 as char;
                    if shift {
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
                        })
                    } else {
                        //I think all alphabetical inputs are UPPERCASE.
                        Key::Char(vk.to_ascii_lowercase())
                    }
                }
                _ => Key::Unknown(vk as u16),
            }
        }
        _ => {
            unsafe { wnd_proc(msg.hwnd, msg.message, msg.w_param, msg.l_param) };
            return None;
        }
    };

    Some(Event::Input(key, modifiers))
}
