use crate::*;
use std::sync::Once;

pub const WH_MOUSE_LL: i32 = 14;

#[repr(C)]
#[derive(Debug, Default)]
pub struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub mouseData: DWORD,
    pub flags: DWORD,
    pub time: DWORD,
    pub dwExtraInfo: usize,
}

pub type HOOKPROC =
    Option<unsafe extern "system" fn(code: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT>;

#[link(name = "user32")]
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> *mut c_void;
    pub fn SetWindowsHookExA(
        hook: i32,
        lpfn: HOOKPROC,
        hmod: *mut c_void,
        thread_id: u32,
    ) -> *mut c_void;
    pub fn CallNextHookEx(hhk: *mut c_void, code: i32, wParam: usize, lParam: isize) -> isize;
    pub fn UnhookWindowsHookEx(hhk: *mut c_void) -> i32;
    pub fn GetAsyncKeyState(key: i32) -> i16;
    pub fn PostThreadMessageA(idThread: DWORD, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn GetCurrentThreadId() -> u32;
}

pub static mut HOOK: *mut c_void = core::ptr::null_mut();
pub static mut ONCE: Once = Once::new();

pub const USER_MOUSEMOVE: u32 = WM_USER + 1;
pub const USER_MOUSEWHEEL: u32 = WM_USER + 2;
pub const USER_LBUTTONDOWN: u32 = WM_USER + 3;
pub const USER_LBUTTONUP: u32 = WM_USER + 4;
pub const USER_LBUTTONDBLCLK: u32 = WM_USER + 5;
pub const USER_RBUTTONDOWN: u32 = WM_USER + 6;
pub const USER_RBUTTONUP: u32 = WM_USER + 7;
pub const USER_RBUTTONDBLCLK: u32 = WM_USER + 8;
pub const USER_MBUTTONDOWN: u32 = WM_USER + 9;
pub const USER_MBUTTONUP: u32 = WM_USER + 10;
pub const USER_MBUTTONDBLCLK: u32 = WM_USER + 11;
pub const USER_XBUTTONDOWN: u32 = WM_USER + 12;
pub const USER_XBUTTONUP: u32 = WM_USER + 13;
pub const USER_XBUTTONDBLCLK: u32 = WM_USER + 14;

pub unsafe extern "system" fn mouse_proc(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let msg = match w_param as u32 {
            WM_MOUSEMOVE => USER_MOUSEMOVE,
            WM_MOUSEWHEEL => {
                let thread_id = GetCurrentThreadId();
                //For some reaons the mouse data is not working when I send it over.
                let mouse = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
                //Just pass the delta into the w_param and convert it back to i16.
                //Should be fine right? 😏
                let delta = (mouse.mouseData >> 16) as i16;
                assert!(
                    PostThreadMessageA(thread_id, USER_MOUSEWHEEL, delta as usize, l_param) != 0
                );
                return CallNextHookEx(HOOK, code, w_param, l_param);
            }
            WM_LBUTTONDOWN => USER_LBUTTONDOWN,
            WM_LBUTTONUP => USER_LBUTTONUP,
            WM_LBUTTONDBLCLK => USER_LBUTTONDBLCLK,
            WM_RBUTTONDOWN => USER_RBUTTONDOWN,
            WM_RBUTTONUP => USER_RBUTTONUP,
            WM_RBUTTONDBLCLK => USER_RBUTTONDBLCLK,
            WM_MBUTTONDOWN => USER_MBUTTONDOWN,
            WM_MBUTTONUP => USER_MBUTTONUP,
            WM_MBUTTONDBLCLK => USER_MBUTTONDBLCLK,
            WM_XBUTTONDOWN => USER_XBUTTONDOWN,
            WM_XBUTTONUP => USER_XBUTTONUP,
            WM_XBUTTONDBLCLK => USER_XBUTTONDBLCLK,
            _ => return CallNextHookEx(HOOK, code, w_param, l_param),
        };

        let thread_id = GetCurrentThreadId();
        assert!(PostThreadMessageA(thread_id, msg, w_param, l_param) != 0);
    }

    CallNextHookEx(HOOK, code, w_param, l_param)
}

pub fn poll_global_events() -> Option<Event> {
    unsafe {
        ONCE.call_once(|| {
            let instance = GetModuleHandleA(core::ptr::null());
            HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
            assert!(!HOOK.is_null());
        });

        let mut msg: MSG = core::mem::zeroed();
        let result = PeekMessageA(&mut msg, 0, 0, 0, PM_REMOVE);

        if msg.message > WM_USER {
            handle_mouse_msg(msg, result)
        } else {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
            None
        }
    }
}

pub fn wait_for_global_events() -> Option<Event> {
    unsafe {
        ONCE.call_once(|| {
            let instance = GetModuleHandleA(core::ptr::null());
            HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
            assert!(!HOOK.is_null());
        });

        let mut msg: MSG = core::mem::zeroed();
        let result = GetMessageA(&mut msg, 0, 0, 0);

        if msg.message > WM_USER {
            handle_mouse_msg(msg, result)
        } else {
            None
        }
    }
}

pub fn handle_mouse_msg(msg: MSG, result: i32) -> Option<Event> {
    match result {
        -1 => {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageA`, error code: {}", last_error);
        }
        0 => return None,
        _ => {}
    }

    let ptr = msg.l_param as *const MSLLHOOKSTRUCT;
    if ptr.is_null() {
        return None;
    }

    let mouse = unsafe { &*(msg.l_param as *const MSLLHOOKSTRUCT) };
    let modifiers = modifiers();

    match msg.message {
        USER_MOUSEMOVE => Some(Event::MouseMoveGlobal(mouse.pt.x as i32, mouse.pt.y as i32)),
        USER_MOUSEWHEEL => {
            const WHEEL_DELTA: i16 = 120;
            let delta = (msg.w_param as i16) as f32 / WHEEL_DELTA as f32;
            if delta >= 0.0 {
                Some(Event::Input(Key::ScrollUp, modifiers))
            } else {
                Some(Event::Input(Key::ScrollDown, modifiers))
            }
        }
        USER_LBUTTONDOWN => Some(Event::Input(Key::LeftMouseDown, modifiers)),
        USER_LBUTTONUP => Some(Event::Input(Key::LeftMouseUp, modifiers)),
        USER_LBUTTONDBLCLK => Some(Event::Input(Key::LeftMouseDoubleClick, modifiers)),
        USER_RBUTTONDOWN => Some(Event::Input(Key::RightMouseDown, modifiers)),
        USER_RBUTTONUP => Some(Event::Input(Key::RightMouseUp, modifiers)),
        USER_RBUTTONDBLCLK => Some(Event::Input(Key::RightMouseDoubleClick, modifiers)),
        USER_MBUTTONDOWN => Some(Event::Input(Key::MiddleMouseDown, modifiers)),
        USER_MBUTTONUP => Some(Event::Input(Key::MiddleMouseUp, modifiers)),
        USER_MBUTTONDBLCLK => Some(Event::Input(Key::MiddleMouseDoubleClick, modifiers)),
        USER_XBUTTONDOWN => {
            let btn = HIWORD(mouse.mouseData);
            match btn {
                1 => Some(Event::Input(Key::Mouse4Down, modifiers)),
                2 => Some(Event::Input(Key::Mouse5Down, modifiers)),
                _ => None,
            }
        }
        USER_XBUTTONUP => {
            let btn = HIWORD(mouse.mouseData);
            match btn {
                1 => Some(Event::Input(Key::Mouse4Up, modifiers)),
                2 => Some(Event::Input(Key::Mouse5Up, modifiers)),
                _ => None,
            }
        }
        USER_XBUTTONDBLCLK => {
            let btn = HIWORD(mouse.mouseData);
            match btn {
                1 => Some(Event::Input(Key::Mouse4DoubleClick, modifiers)),
                2 => Some(Event::Input(Key::Mouse5DoubleClick, modifiers)),
                _ => None,
            }
        }
        _ => None,
    }
}

///https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn is_down(virtual_key: i32) -> bool {
    (unsafe { GetAsyncKeyState(virtual_key) } & 0x8000u16 as i16) != 0
}

pub fn get_mouse_position() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    let _ = unsafe { GetCursorPos(&mut point) };

    (point.x, point.y)
}

pub fn get_physical_mouse_position() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    let _ = unsafe { GetPhysicalCursorPos(&mut point) };

    (point.x, point.y)
}
