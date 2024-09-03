#![allow(non_snake_case, static_mut_refs)]
mod constants;
mod gdi;
mod window;

use core::ptr::null;
use core::ptr::null_mut;
use std::sync::atomic::AtomicUsize;

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

use core::ffi::c_void;
use std::sync::atomic::Ordering;

pub static mut WINDOW_COUNT: AtomicUsize = AtomicUsize::new(0);

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

    pub const fn new() -> Self {
        Self {
            hwnd: 0,
            message: 0,
            w_param: 0,
            l_param: 0,
            time: 0,
            pt: Point { x: 0, y: 0 },
        }
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

#[link(name = "user32")]
extern "system" {
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
        msg: *mut MSG,
        hwnd: isize,
        msg_filter_min: u32,
        msg_filter_max: u32,
        remove_msg: u32,
    ) -> i32;
    pub fn GetMessageA(msg: *mut MSG, hwnd: isize, msg_filter_min: u32, msg_filter_max: u32)
        -> i32;
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16;
    pub fn DispatchMessageA(lpMsg: *const MSG) -> isize;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn GetLastError() -> u32;
    pub fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
    pub fn LoadLibraryA(lpFileName: *const i8) -> *mut c_void;
    pub fn GetDC(hwnd: isize) -> *mut c_void;
    pub fn LoadCursorW(hInstance: *mut c_void, lpCursorName: *const u16) -> *mut c_void;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn GetCursorPos(point: *mut Point) -> i32;
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
    pub fn AdjustWindowRectEx(lpRect: *mut RECT, dwStyle: u32, bMenu: i32, dwExStyle: u32) -> i32;
    pub fn GetDesktopWindow() -> isize;
    pub fn GetWindowRect(hwnd: isize, lpRect: *mut RECT) -> i32;
    pub fn GetClientRect(hwnd: isize, lpRect: *mut RECT) -> i32;
    pub fn ClientToScreen(hwnd: isize, lpPoint: *mut Point) -> i32;
    pub fn ValidateRect(hwnd: isize, lpRect: *const RECT) -> i32;
    pub fn SetWindowPos(
        hWnd: isize,
        hWndInsertAfter: isize,
        X: i32,
        Y: i32,
        cx: i32,
        cy: i32,
        uFlags: u32,
    ) -> i32;
    pub fn DwmGetWindowAttribute(
        hWnd: isize,
        dwAttribute: u32,
        pvAttribute: *mut c_void,
        cbAttribute: u32,
    ) -> i32;
    pub fn GetSystemMetricsForDpi(nIndex: i32, dpi: u32) -> i32;
    pub fn SetThreadDpiAwarenessContext(dpiContext: DpiAwareness) -> isize;
    // pub fn GetThreadDpiAwarenessContext() -> isize;
    pub fn GetDpiForWindow(hwnd: isize) -> u32;
    pub fn ReleaseCapture() -> i32;
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
    Mouse(i32, i32),
    Move,
    // This event is only triggerd after a resize, so it's not very useful.
    // Resize,
    Dpi(usize),
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

pub fn mouse_pos() -> (i32, i32) {
    let mut point = Point { x: 0, y: 0 };
    let _ = unsafe { GetCursorPos(&mut point) };

    (point.x, point.y)
}

///To get the window bounds excluding the drop shadow, use DwmGetWindowAttribute, specifying DWMWA_EXTENDED_FRAME_BOUNDS. Note that unlike the Window Rect, the DWM Extended Frame Bounds are not adjusted for DPI. Getting the extended frame bounds can only be done after the window has been shown at least once.
pub fn screen_area_no_shadow(_hwnd: isize) -> RECT {
    todo!();
}

///WinRect coordiantes can be negative.
pub fn screen_area(hwnd: isize) -> RECT {
    let mut rect = RECT::default();
    let _ = unsafe { GetWindowRect(hwnd, &mut rect) };
    rect
}

///WinRect coordiantes *should* never be negative.
pub fn client_area(hwnd: isize) -> RECT {
    let mut rect = RECT::default();
    let _ = unsafe { GetClientRect(hwnd, &mut rect) };
    rect
}

/// The desktop window is the area on top of which other windows are painted.
pub fn desktop_area() -> RECT {
    unsafe { client_area(GetDesktopWindow()) }
}

// pub fn is_maximized(window: HWND) -> bool {
//     unsafe {
//         let mut placement: WINDOWPLACEMENT = mem::zeroed();
//         placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
//         GetWindowPlacement(window, &mut placement);
//         placement.showCmd == SW_MAXIMIZE
//     }
// }

pub fn event(hwnd: Option<isize>) -> Option<Event> {
    let mut msg = MSG::new();
    let result = unsafe { PeekMessageA(&mut msg, hwnd.unwrap_or_default(), 0, 0, PM_REMOVE) };
    handle_msg(msg, result)
}

pub fn event_blocking(hwnd: Option<isize>) -> Option<Event> {
    let mut msg = MSG::new();
    let result = unsafe { GetMessageA(&mut msg, hwnd.unwrap_or_default(), 0, 0) };
    handle_msg(msg, result)
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
fn handle_msg(msg: MSG, result: i32) -> Option<Event> {
    unsafe {
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
                        VK_OEM_PLUS if shift => {
                            return Some(Event::Input(Key::Char('+'), modifiers))
                        }
                        VK_OEM_MINUS if shift => {
                            return Some(Event::Input(Key::Char('_'), modifiers))
                        }
                        VK_OEM_3 if shift => return Some(Event::Input(Key::Char('~'), modifiers)),
                        VK_OEM_4 if shift => return Some(Event::Input(Key::Char('{'), modifiers)),
                        VK_OEM_6 if shift => return Some(Event::Input(Key::Char('}'), modifiers)),
                        VK_OEM_5 if shift => return Some(Event::Input(Key::Char('|'), modifiers)),
                        VK_OEM_1 if shift => return Some(Event::Input(Key::Char(':'), modifiers)),
                        VK_OEM_7 if shift => return Some(Event::Input(Key::Char('"'), modifiers)),
                        VK_OEM_COMMA if shift => {
                            return Some(Event::Input(Key::Char('<'), modifiers))
                        }
                        VK_OEM_PERIOD if shift => {
                            return Some(Event::Input(Key::Char('>'), modifiers))
                        }
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
}

///Only works on Windows 1809 and above.
pub unsafe fn set_dark_mode(hwnd: isize) -> Result<(), &'static str> {
    const WCA_USEDARKMODECOLORS: u32 = 26;
    const DARK_MODE: i32 = 1;
    // const LIGHT_MODE: i32 = 0;

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

    #[link(name = "ntdll")]
    extern "system" {
        fn RtlGetVersion(version: *mut OSVERSIONINFOW) -> i32;
    }

    let mut v = OSVERSIONINFOW {
        dw_osversion_info_size: 0,
        dw_major_version: 0,
        dw_minor_version: 0,
        dw_build_number: 0,
        dw_platform_id: 0,
        sz_csdversion: [0; 128],
    };
    let status = RtlGetVersion(&mut v);

    //Check if this version of windows supports `SetWindowCompositionAttribute`.
    if v.dw_build_number < 17763 || status < 0 {
        return Err("Window version must be 1809 or above.");
    }

    let user32 = LoadLibraryA("user32.dll\0".as_ptr() as *const i8);
    let fn_ptr = GetProcAddress(
        user32,
        "SetWindowCompositionAttribute\0".as_ptr() as *const i8,
    );
    let SetWindow: fn(isize, *mut WINDOWCOMPOSITIONATTRIBDATA) -> i32 = std::mem::transmute(fn_ptr);

    //This must be mutable.
    let mut dark_mode: i32 = DARK_MODE;
    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WCA_USEDARKMODECOLORS,
        data: &mut dark_mode as *mut i32 as _,
        size: 4,
    };

    if SetWindow(hwnd, &mut data) != 0 {
        Ok(())
    } else {
        Err("Call to SetWindowCompositionAttribute failed.")
    }
}
