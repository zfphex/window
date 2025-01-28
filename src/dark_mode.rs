use crate::*;

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