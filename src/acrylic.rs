//! I do not recommend using this.
use crate::*;
use core::mem::size_of;

pub const ACCENT_DISABLED: u32 = 0;
pub const ACCENT_ENABLE_TRANSPARENTGRADIENT: u32 = 2;
pub const ACCENT_ENABLE_BLURBEHIND: u32 = 3;
pub const ACCENT_ENABLE_ACRYLICBLURBEHIND: u32 = 4;
pub const ACCENT_ENABLE_HOSTBACKDROP: u32 = 5;
pub const WCA_ACCENT_POLICY: u32 = 19;

pub unsafe fn set_acrylic(hwnd: isize, state: u32, color: u32) {
    #[repr(C)]
    struct WINDOWCOMPOSITIONATTRIBDATA {
        attrib: u32,
        data: *mut core::ffi::c_void,
        size: usize,
    }

    #[repr(C)]
    struct ACCENT_POLICY {
        nAccentState: u32,
        nFlags: u32,
        gradientColor: u32, // ARGB
        animationId: u32,
    }

    let user32 = LoadLibraryA("user32.dll\0".as_ptr() as *const i8);
    let proc = GetProcAddress(
        user32,
        "SetWindowCompositionAttribute\0".as_ptr() as *const i8,
    );
    let SetWindow: fn(isize, *mut WINDOWCOMPOSITIONATTRIBDATA) -> i32 = core::mem::transmute(proc);

    let accent = ACCENT_POLICY {
        nAccentState: state,
        nFlags: 0,
        gradientColor: color, // 0xAABBGGRR
        animationId: 0,
    };

    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WCA_ACCENT_POLICY,
        data: &accent as *const _ as *mut core::ffi::c_void,
        size: size_of::<ACCENT_POLICY>(),
    };

    assert!(SetWindow(hwnd, &mut data) != 0);
}
