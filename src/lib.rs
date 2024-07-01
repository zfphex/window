#![allow(non_snake_case, static_mut_refs)]
mod constants;
mod gdi;
mod window;

pub mod queue_test;

use core::ptr::addr_of_mut;
use core::ptr::null;
use core::ptr::null_mut;

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
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl RECT {
    pub const fn width(&self) -> i32 {
        self.right - self.left
    }
    pub const fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            left: x,
            top: y,
            right: width,
            bottom: height,
        }
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

impl MSG {
    #[inline]
    pub fn low_order_l(&self) -> isize {
        self.l_param >> 16 & 0xFFFF
    }

    #[inline]
    pub fn high_order_l(&self) -> isize {
        self.l_param & 0xFFFF
    }

    #[inline]
    pub fn low_order_w(&self) -> usize {
        self.w_param >> 16 & 0xFFFF
    }

    #[inline]
    pub fn high_order_w(&self) -> usize {
        self.w_param & 0xFFFF
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct WindowInfo {
    pub size: u32,
    pub window: RECT,
    pub client: RECT,
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
    ///GetMessage blocks until a message is posted before returning.
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
    pub fn DispatchMessageA(lpMsg: *const MSG) -> isize;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn GetLastError() -> u32;
    pub fn GetProcAddress(hModule: *mut VOID, lpProcName: *const i8) -> *mut VOID;
    pub fn LoadLibraryA(lpFileName: *const i8) -> *mut VOID;
    ///The GetDC function retrieves a handle to a device context (DC) for the client area of a specified window or for the entire screen.
    pub fn GetDC(hwnd: isize) -> *mut VOID;
    pub fn LoadCursorW(hInstance: *mut VOID, lpCursorName: *const u16) -> *mut VOID;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn GetCursorPos(lpPoint: *mut Point) -> i32;

    //Window Functions
    pub fn DefWindowProcA(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize;
    pub fn GetWindow(hwnd: isize, uCmd: u32) -> isize;
    pub fn DestroyWindow(hwnd: isize) -> i32;
    pub fn GetForegroundWindow() -> isize;

    pub fn GetWindowLongPtrW(hwnd: isize, nIndex: i32) -> isize;
    pub fn SetWindowLongPtrW(hwnd: isize, nIndex: i32, dwNewLong: isize) -> isize;

    pub fn GetWindowLongPtrA(hwnd: isize, nIndex: i32) -> isize;
    pub fn SetWindowLongPtrA(hwnd: isize, nIndex: i32, dwNewLong: isize) -> isize;

    pub fn GetWindowLongA(hwnd: isize, nIndex: i32) -> LONG;
    pub fn SetWindowLongA(hwnd: isize, nIndex: i32, dwNewLong: LONG) -> LONG;

    pub fn ShowWindow(hwnd: isize, nCmdShow: i32) -> BOOL;
    pub fn GetWindowInfo(hwnd: isize, pwi: *mut WindowInfo) -> i32;
    ///Calculates the required size of the window rectangle, based on the desired size of the client rectangle. The window rectangle can then be passed to the CreateWindowEx function to create a window whose client area is the desired size.
    pub fn AdjustWindowRectEx(lpRect: *mut RECT, dwStyle: u32, bMenu: i32, dwExStyle: u32) -> i32;
    pub fn GetDesktopWindow() -> isize;

    ///Retrieves the screen coordinates of the specified window.
    pub fn GetWindowRect(hwnd: isize, lpRect: *mut RECT) -> i32;
    ///Retrieves the coordinates of a window's client area.
    pub fn GetClientRect(hwnd: isize, lpRect: *mut RECT) -> i32;
    pub fn ClientToScreen(hwnd: isize, lpPoint: *mut Point) -> BOOL;
    pub fn ValidateRect(hwnd: isize, lpRect: *const RECT) -> i32;

    pub fn DwmGetWindowAttribute(
        hWnd: isize,
        dwAttribute: u32,
        pvAttribute: *mut VOID,
        cbAttribute: u32,
    ) -> i32;

    pub fn GetSystemMetricsForDpi(nIndex: i32, dpi: u32) -> i32;

    //TODO: Remove
    // pub fn SetRect(lprc: *mut WinRect, xLeft: i32, yTop: i32, xRight: i32, yBottom: i32) -> BOOL;

    pub fn SetThreadDpiAwarenessContext(dpiContext: DPI_AWARENESS_CONTEXT)
        -> DPI_AWARENESS_CONTEXT;
    pub fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
    pub fn GetDpiForWindow(hwnd: isize) -> u32;
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
    Mouse(i32, i32),

    LeftMouseDown,
    MiddleMouseDown,
    RightMouseDown,
    LeftMouseUp,
    MiddleMouseUp,
    RightMouseUp,
    ScrollUp,
    ScrollDown,

    Unknown(u16),
    LeftMouseDoubleClick,
    RightMouseDoubleClick,
    Mouse4Up,
    Mouse5Up,
    Mouse4Down,
    Mouse5Down,
    Mouse4DoubleClick,
    Mouse5DoubleClick,
    MiddleMouseDoubleClick,
    Move,
    Resize,
    Dpi(usize),
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
pub fn event(hwnd: Option<isize>) -> Option<Event> {
    unsafe {
        if QUIT {
            return Some(Event::Quit);
        }

        //Note that some messages like WM_MOVE and WM_SIZE will not be included here.
        //wndproc must be used for window related messages.
        let hwnd = hwnd.unwrap_or_default();
        let result = PeekMessageA(addr_of_mut!(MSG), hwnd, 0, 0, PM_REMOVE);

        //Mouse position.
        // let (x, y) = (MSG.pt.x, MSG.pt.y);

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

///Only works on Windows 1809 and above.
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
