#![allow(non_snake_case, static_mut_refs)]
mod constants;
mod dark_mode;
mod event;
mod gdi;
mod window;

pub use constants::*;
pub use dark_mode::*;
pub use event::*;
pub use gdi::*;
pub use window::*;

use core::{
    ffi::c_void,
    ptr::{null, null_mut},
};
use std::sync::atomic::{AtomicUsize, Ordering};

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
    pub fn GetPixel(hdc: *mut c_void, x: i32, y: i32) -> u32;
    pub fn GetFocus() -> HWND;

    pub fn WindowFromPoint(point: Point) -> HWND;
    pub fn GetDeviceCaps(hdc: *mut c_void, index: i32) -> i32;
    pub fn GetSystemMetrics(nIndex: i32) -> i32;

    pub fn LoadCursorW(hInstance: *mut c_void, lpCursorName: *const u16) -> *mut c_void;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn GetCursorPos(point: *mut Point) -> i32;
    pub fn GetPhysicalCursorPos(point: *mut Point) -> i32;
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
    pub fn MoveWindow(
        hWnd: HWND,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        bRepaint: BOOL,
    ) -> BOOL;
    pub fn DwmGetWindowAttribute(
        hWnd: isize,
        dwAttribute: u32,
        pvAttribute: *mut c_void,
        cbAttribute: u32,
    ) -> i32;

    pub fn GetSystemMetricsForDpi(nIndex: i32, dpi: u32) -> i32;

    pub fn GetThreadDpiAwarenessContext() -> *mut c_void;
    pub fn SetThreadDpiAwarenessContext(dpi_context: *mut c_void) -> isize;

    pub fn GetWindowDpiAwarenessContext(hwnd: isize) -> *mut c_void;

    pub fn GetDpiForWindow(hwnd: isize) -> u32;
    pub fn ReleaseCapture() -> i32;
}

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

pub fn physical_mouse_pos() -> (i32, i32) {
    let mut point = Point { x: 0, y: 0 };
    let _ = unsafe { GetPhysicalCursorPos(&mut point) };

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

#[inline]
pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

#[inline]
pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}
