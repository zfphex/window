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
}

pub static mut HOOK: *mut c_void = core::ptr::null_mut();

pub unsafe extern "system" fn mouse_proc(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let mouse_struct = &*(l_param as *const MSLLHOOKSTRUCT);

        match w_param as UINT {
            WM_LBUTTONDOWN => println!("Left mouse button clicked at: {:?}", mouse_struct.pt),
            WM_RBUTTONDOWN => println!("Right mouse button clicked at: {:?}", mouse_struct.pt),
            WM_MBUTTONDOWN => println!("Middle mouse button clicked at: {:?}", mouse_struct.pt),
            _ => {}
        }
    }

    CallNextHookEx(HOOK, code, w_param, l_param)
}

///https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn is_down(virtual_key: i32) -> bool {
    (unsafe { GetAsyncKeyState(virtual_key) } & 0x8000u16 as i16) != 0
}
