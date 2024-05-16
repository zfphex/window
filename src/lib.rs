#![allow(non_snake_case)]
mod constants;
mod gdi;
mod window;

use std::ptr::addr_of_mut;

pub use constants::*;
pub use gdi::*;
pub use window::*;

pub type HWND = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type WORD = u16;
pub type DWORD = u32;
pub type BOOL = i32;
pub type UINT = u32;
pub type LONG = i32;
pub type LPCSTR = *const i8;
pub type LPCWSTR = *const u16;

// pub enum VOID {}
// pub type VOID = *const ();
pub type VOID = std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct MSG {
    pub hwnd: isize,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: Point,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct WindowInfo {
    pub size: u32,
    pub window: Rect,
    pub client: Rect,
    pub style: u32,
    pub ex_style: u32,
    pub window_status: u32,
    pub window_borders_x: u32,
    pub window_borders_y: u32,
    pub window_type: u16,
    pub creator_version: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WNDCLASSA {
    pub style: u32,
    pub wnd_proc: Option<
        unsafe extern "system" fn(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize,
    >,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: isize,
    pub icon: isize,
    pub cursor: isize,
    pub background: isize,
    pub menu_name: *const u8,
    pub class_name: *const u8,
}

impl Default for WNDCLASSA {
    fn default() -> Self {
        Self {
            style: Default::default(),
            wnd_proc: Default::default(),
            cls_extra: Default::default(),
            wnd_extra: Default::default(),
            instance: Default::default(),
            icon: Default::default(),
            cursor: Default::default(),
            background: Default::default(),
            menu_name: unsafe { std::mem::zeroed() },
            class_name: unsafe { std::mem::zeroed() },
        }
    }
}

static mut MSG: MSG = MSG {
    hwnd: 0,
    message: 0,
    w_param: 0,
    l_param: 0,
    time: 0,
    pt: Point { x: 0, y: 0 },
};

static mut QUIT: bool = false;

#[link(name = "user32")]
extern "system" {
    ///Return value
    ///
    ///Type: `HWND`
    ///
    ///If the function succeeds, the return value is a handle to the new window.
    ///
    ///If the function fails, the return value is `NULL`. To get extended error information, call GetLastError.
    ///
    ///This function typically fails for one of the following reasons:
    ///
    ///- an invalid parameter value
    ///- the system class was registered by a different module
    ///- The WH_CBT hook is installed and returns a failure code
    ///- if one of the controls in the dialog template is not registered, or its window window procedure fails WM_CREATE or WM_NCCREATE
    pub fn CreateWindowExA(
        dwexstyle: u32,
        lpclassname: *const u8,
        lpwindowname: *const u8,
        dwstyle: u32,
        x: i32,
        y: i32,
        nwidth: i32,
        nheight: i32,
        hwndparent: isize,
        hmenu: isize,
        hinstance: isize,
        lpparam: *const std::ffi::c_void,
    ) -> isize;
    pub fn PeekMessageA(
        lpmsg: *mut MSG,
        hwnd: isize,
        wmsgfiltermin: u32,
        wmsgfiltermax: u32,
        wremovemsg: u32,
    ) -> i32;
    pub fn GetMessageA(
        lpMsg: *const MSG,
        hWnd: isize,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;
    /// Indicates to the system that a thread has made a request to terminate (quit).
    /// It is typically used in response to a WM_DESTROY message.
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16;
    pub fn AdjustWindowRectEx(lpRect: *mut Rect, dwStyle: u32, bMenu: i32, dwExStyle: u32) -> i32;
    pub fn DestroyWindow(hWnd: isize) -> i32;
    pub fn DefWindowProcA(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize;
    pub fn DispatchMessageA(lpMsg: *const MSG) -> isize;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn GetLastError() -> u32;
    pub fn GetWindow(hwnd: isize, uCmd: u32) -> isize;
    pub fn GetForegroundWindow() -> isize;
    pub fn GetProcAddress(hModule: *mut VOID, lpProcName: *const i8) -> *mut VOID;
    pub fn LoadLibraryA(lpFileName: *const i8) -> *mut VOID;
    pub fn GetWindowLongPtrA(hwnd: isize, nIndex: i32) -> isize;
    pub fn ValidateRect(hwnd: isize, lpRect: *const Rect) -> i32;
    pub fn GetWindowRect(hwnd: isize, lpRect: *mut Rect) -> i32;
    pub fn GetWindowInfo(hwnd: isize, pwi: *mut WindowInfo) -> i32;
    pub fn SetWindowLongA(hWnd: HWND, nIndex: i32, dwNewLong: LONG) -> LONG;
    pub fn GetWindowLongA(hWnd: HWND, nIndex: i32) -> LONG;
    pub fn ClientToScreen(hWnd: HWND, lpPoint: *mut Point) -> BOOL;
    pub fn GetDC(hwnd: isize) -> *mut VOID;
    pub fn LoadCursorW(hInstance: *mut VOID, lpCursorName: *const u16) -> *mut VOID;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn GetCursorPos(lpPoint: *mut Point) -> i32;
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

// pub struct Key {
//     code: u32,
//     modifier: Modifier,
// }

const F24: i32 = VK_F1 + 23;

#[derive(Debug, PartialEq)]
pub enum Event {
    Quit,
    // Key(char, Modifier),
    //Key
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

    //(0, 0) is top left of window.
    Mouse(usize, usize),

    LeftMouseDown,
    MiddleMouseDown,
    RightMouseDown,
    LeftMouseUp,
    MiddleMouseUp,
    RightMouseUp,
    ScrollUp,
    ScrollDown,

    Unknown(u16),
}

pub fn mouse_pos() -> (i32, i32) {
    let mut point = Point { x: 0, y: 0 };
    let _ = unsafe { GetCursorPos(&mut point) };

    (point.x, point.y)
}

pub struct Modifiers {
    pub control: bool,
    pub shift: bool,
    pub alt: bool,
    pub win: bool,
}

//https://github.com/makepad/makepad/blob/69bef6bab686284e1e3ab83ee803f29c5c9f40e5/platform/src/os/windows/win32_window.rs#L765
fn modifiers() -> Modifiers {
    unsafe {
        Modifiers {
            control: GetKeyState(VK_CONTROL) & 0x80 > 0,
            shift: GetKeyState(VK_SHIFT) & 0x80 > 0,
            alt: GetKeyState(VK_MENU) & 0x80 > 0,
            win: GetKeyState(VK_LWIN) & 0x80 > 0 || GetKeyState(VK_RWIN) & 0x80 > 0,
        }
    }
}

pub fn event() -> Option<Event> {
    unsafe {
        if QUIT {
            return Some(Event::Quit);
        }

        let result = PeekMessageA(addr_of_mut!(MSG), 0, 0, 0, PM_REMOVE);
        match result {
            0 => None,
            _ => match MSG.message {
                WM_MOUSEMOVE => {
                    let x = MSG.l_param & 0xFFFF;
                    let y = MSG.l_param >> 16 & 0xFFFF;
                    Some(Event::Mouse(x as usize, y as usize))
                }
                //TODO: Double clicks.
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

                        VK_F1..=F24 => return Some(Event::Function((vk - VK_F1 as i32 + 1) as u8)),
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

pub fn event_blocking() -> Option<Event> {
    let message_result = unsafe { GetMessageA(addr_of_mut!(MSG), 0, 0, 0) };

    match message_result {
        -1 => {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageA`, error code: {}", last_error);
        }
        0 => Some(Event::Quit),
        _ => {
            //Handle message here.
            unsafe {
                TranslateMessage(addr_of_mut!(MSG));
                DispatchMessageA(addr_of_mut!(MSG));
            }
            None
        }
    }
}

//TODO: Find a way to toggle this at runtime?
///Only works on Windows 17763 and above.
pub unsafe fn set_dark_mode(hwnd: isize) -> bool {
    const WCA_USEDARKMODECOLORS: u32 = 26;

    #[repr(C)]
    struct WINDOWCOMPOSITIONATTRIBDATA {
        attrib: u32,
        data: *mut std::ffi::c_void,
        size: usize,
    }

    #[repr(C)]
    struct OSVERSIONINFOW {
        pub dw_osversion_info_size: u32,
        pub dw_major_version: u32,
        pub dw_minor_version: u32,
        pub dw_build_number: u32,
        pub dw_platform_id: u32,
        pub sz_csdversion: [u16; 128],
    }

    //Check if this version of windows supports `SetWindowCompositionAttribute`.
    let nt = LoadLibraryA("ntdll.dll\0".as_ptr() as *const i8);
    let func = GetProcAddress(nt, "RtlGetVersion\0".as_ptr() as *const i8);
    let get_version: fn(*mut OSVERSIONINFOW) -> i32 = std::mem::transmute(func);
    let mut v: OSVERSIONINFOW = OSVERSIONINFOW {
        dw_osversion_info_size: 0,
        dw_major_version: 0,
        dw_minor_version: 0,
        dw_build_number: 0,
        dw_platform_id: 0,
        sz_csdversion: [0; 128],
    };
    let status = get_version(&mut v);

    if v.dw_build_number < 17763 || status < 0 {
        return false;
    }

    let user32 = LoadLibraryA("user32.dll\0".as_ptr() as *const i8);
    let func = GetProcAddress(
        user32,
        "SetWindowCompositionAttribute\0".as_ptr() as *const i8,
    );
    let set_window: fn(isize, *mut WINDOWCOMPOSITIONATTRIBDATA) -> i32 = std::mem::transmute(func);
    let mut dark_mode: i32 = 1;
    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WCA_USEDARKMODECOLORS,
        data: &mut dark_mode as *mut i32 as _,
        size: 4,
    };

    set_window(hwnd, &mut data) != 0
}

pub fn get_hinstance() -> isize {
    // Gets the instance handle by taking the address of the
    // pseudo-variable created by the microsoft linker:
    // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483

    // This is preferred over GetModuleHandle(NULL) because it also works in DLLs:
    // https://stackoverflow.com/questions/21718027/getmodulehandlenull-vs-hinstance
    #[repr(C, packed(2))]
    pub struct ImageDosHeader {
        pub e_magic: u16,
        pub e_cblp: u16,
        pub e_cp: u16,
        pub e_crlc: u16,
        pub e_cparhdr: u16,
        pub e_minalloc: u16,
        pub e_maxalloc: u16,
        pub e_ss: u16,
        pub e_sp: u16,
        pub e_csum: u16,
        pub e_ip: u16,
        pub e_cs: u16,
        pub e_lfarlc: u16,
        pub e_ovno: u16,
        pub e_res: [u16; 4],
        pub e_oemid: u16,
        pub e_oeminfo: u16,
        pub e_res2: [u16; 10],
        pub e_lfanew: i32,
    }

    extern "C" {
        static __ImageBase: ImageDosHeader;
    }

    unsafe { &__ImageBase as *const _ as _ }
}
