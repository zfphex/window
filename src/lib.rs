#![allow(
    non_snake_case,
    static_mut_refs,
    non_camel_case_types,
    unused_variables
)]

use core::{
    ffi::c_void,
    ptr::{null, null_mut},
};

mod acrylic;
mod clipboard;
mod constants;
mod dark_theme;
mod debug;
mod dwm;
mod event;
mod fps;
mod gdi;
mod global_input;
mod input;
mod monitor;
mod tray;
mod wgl;
mod window;

pub use acrylic::*;
pub use clipboard::*;
pub use constants::*;
pub use dark_theme::*;
pub use debug::*;
pub use dwm::*;
pub use event::*;
pub use fps::*;
pub use gdi::*;
pub use global_input::*;
pub use input::*;
pub use monitor::*;
pub use tray::*;
pub use wgl::*;
pub use window::*;

pub type BYTE = u8;
pub type HDC = *mut c_void;
pub type HANDLE = *mut c_void;
pub type HWND = isize;
pub type HGLRC = *mut c_void;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type HRESULT = i32;
pub type WORD = u16;
pub type DWORD = u32;
pub type BOOL = i32;
pub type UINT = u32;
pub type LONG = i32;
pub type LPCSTR = *const i8;
pub type LPCWSTR = *const u16;
pub type LPWSTR = *mut u16;

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
    ///Translates virtual-key messages into character messages.
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn GetLastError() -> u32;
    pub fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
    pub fn LoadLibraryA(lpFileName: *const i8) -> *mut c_void;

    pub fn GetDC(hwnd: isize) -> *mut c_void;
    pub fn GetPixel(hdc: *mut c_void, x: i32, y: i32) -> u32;
    pub fn GetFocus() -> HWND;

    pub fn WindowFromPoint(point: POINT) -> HWND;
    pub fn GetDeviceCaps(hdc: *mut c_void, index: i32) -> i32;
    pub fn GetSystemMetrics(nIndex: i32) -> i32;

    pub fn LoadCursorW(hInstance: *mut c_void, lpCursorName: *const u16) -> *mut c_void;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn GetCursorPos(point: *mut POINT) -> i32;
    pub fn GetPhysicalCursorPos(point: *mut POINT) -> i32;
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
    pub fn ClientToScreen(hwnd: isize, lpPoint: *mut POINT) -> i32;
    pub fn ScreenToClient(hwnd: isize, point: *mut POINT) -> i32;
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

    pub fn SetLayeredWindowAttributes(hwnd: isize, color_key: u32, alpha: u8, flags: u32) -> i32;

    pub fn GetSystemMetricsForDpi(nIndex: i32, dpi: u32) -> i32;

    pub fn GetThreadDpiAwarenessContext() -> *mut c_void;
    pub fn SetThreadDpiAwarenessContext(dpi_context: *mut c_void) -> isize;

    pub fn GetWindowDpiAwarenessContext(hwnd: isize) -> *mut c_void;

    pub fn GetDpiForWindow(hwnd: isize) -> u32;
    pub fn ReleaseCapture() -> i32;
    pub fn LoadIconA(hInstance: *mut c_void, lpIconName: *const i8) -> *mut c_void;
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    // pub const fn default() -> Self {
    //     Self {
    //         x: 0,
    //         y: 0,
    //         width: 0,
    //         height: 0,
    //     }
    // }
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub const fn x(mut self, x: usize) -> Self {
        self.x = x;
        self
    }
    pub const fn y(mut self, y: usize) -> Self {
        self.y = y;
        self
    }
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub const fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    pub const fn right(&self) -> usize {
        self.x + self.width
    }
    pub const fn bottom(&self) -> usize {
        self.y + self.height
    }
    pub const fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
    //TODO: Bounds checking
    pub const fn inner(&self, w: usize, h: usize) -> Rect {
        Rect {
            x: self.x + w,
            y: self.y + h,
            width: self.width - 2 * w,
            height: self.height - 2 * h,
        }
    }
    pub const fn from_windows(rect: RECT) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: (rect.right - rect.left) as usize,
            height: (rect.bottom - rect.top) as usize,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct MSG {
    pub hwnd: isize,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: POINT,
}

impl MSG {
    pub const fn new() -> Self {
        Self {
            hwnd: 0,
            message: 0,
            w_param: 0,
            l_param: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
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

pub trait LowHighOrder {
    fn low(self) -> Self;
    fn high(self) -> Self;
}

impl LowHighOrder for usize {
    #[inline]
    fn low(self) -> Self {
        self & 0xffff
    }

    #[inline]
    fn high(self) -> Self {
        (self >> 16) & 0xffff
    }
}

impl LowHighOrder for u32 {
    #[inline]
    fn low(self) -> Self {
        self & 0xffff
    }

    #[inline]
    fn high(self) -> Self {
        (self >> 16) & 0xffff
    }
}

// Icon resource identifiers
pub const IDI_APPLICATION: i32 = 32512;
pub const IDI_HAND: i32 = 32513;
pub const IDI_QUESTION: i32 = 32514;
pub const IDI_EXCLAMATION: i32 = 32515;
pub const IDI_ASTERISK: i32 = 32516;
pub const IDI_WINLOGO: i32 = 32517;
