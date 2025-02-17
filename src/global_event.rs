use std::sync::Once;

use crate::*;

pub const WH_MOUSE_LL: i32 = 14;

#[repr(C)]
#[derive(Debug)]
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

//https://github.com/fulara/mki-rust/blob/master/src/windows/mouse.rs
pub unsafe extern "system" fn mouse_proc(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let thread_id = GetCurrentThreadId();
        //Pass the message back to the user.
        PostThreadMessageA(thread_id, w_param as u32, w_param, l_param);
    }

    CallNextHookEx(HOOK, code, w_param, l_param)
}

pub fn poll_global_event() -> Option<Event> {
    unsafe {
        ONCE.call_once(|| {
            let instance = GetModuleHandleA(core::ptr::null());
            HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
            assert!(!HOOK.is_null());
        });

        let mut msg: MSG = core::mem::zeroed();
        let result = PeekMessageA(&mut msg, 0, 0, 0, PM_REMOVE);
        handle_mouse_msg(msg, result)
    }
}

pub fn wait_for_global_event() -> Option<Event> {
    unsafe {
        ONCE.call_once(|| {
            let instance = GetModuleHandleA(core::ptr::null());
            HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
            assert!(!HOOK.is_null());
        });

        let mut msg: MSG = core::mem::zeroed();
        let result = GetMessageA(&mut msg, 0, 0, 0);
        handle_mouse_msg(msg, result)
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

    let w_param = msg.w_param;
    let ptr = msg.l_param as *const MSLLHOOKSTRUCT;
    assert!(!ptr.is_null());
    let mouse = unsafe { &*ptr };

    let modifiers = modifiers();

    match msg.message {
        WM_MOUSEMOVE => Some(Event::MouseMoveGlobal(mouse.pt.x as i32, mouse.pt.y as i32)),
        WM_MOUSEWHEEL => {
            const WHEEL_DELTA: i16 = 120;
            let value = (w_param >> 16) as i16;
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
            let btn = HIWORD(mouse.mouseData);
            match btn {
                1 => Some(Event::Input(Key::Mouse4Down, modifiers)),
                2 => Some(Event::Input(Key::Mouse5Down, modifiers)),
                _ => None,
            }
        }
        WM_XBUTTONUP => {
            let btn = HIWORD(mouse.mouseData);
            match btn {
                1 => Some(Event::Input(Key::Mouse4Up, modifiers)),
                2 => Some(Event::Input(Key::Mouse5Up, modifiers)),
                _ => None,
            }
        }
        WM_XBUTTONDBLCLK => {
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
