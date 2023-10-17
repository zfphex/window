use once_cell::sync::Lazy;
/// This is a simple implementation of support for Windows Dark Mode,
/// which is inspired by the solution in https://github.com/ysc3839/win32-darkmode
use std::{ffi::c_void, ptr};
use windows_sys::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HWND, NTSTATUS, S_OK},
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryA},
            SystemInformation::OSVERSIONINFOW,
        },
        UI::{
            Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTA},
            Controls::SetWindowTheme,
            WindowsAndMessaging::{SystemParametersInfoA, SPI_GETHIGHCONTRAST},
        },
    },
};

use util::*;

use crate::get_function;

pub mod util {
    use std::{
        ffi::{c_void, OsStr, OsString},
        io,
        iter::once,
        mem,
        ops::BitAnd,
        os::windows::prelude::{OsStrExt, OsStringExt},
        ptr,
        sync::atomic::{AtomicBool, Ordering},
    };

    use once_cell::sync::Lazy;
    use windows_sys::{
        core::{HRESULT, PCWSTR},
        Win32::{
            Foundation::{BOOL, HMODULE, HWND, RECT},
            Graphics::Gdi::{ClientToScreen, HMONITOR},
            System::{
                LibraryLoader::{GetProcAddress, LoadLibraryA},
                SystemServices::IMAGE_DOS_HEADER,
            },
            UI::{
                HiDpi::{DPI_AWARENESS_CONTEXT, MONITOR_DPI_TYPE, PROCESS_DPI_AWARENESS},
                Input::KeyboardAndMouse::GetActiveWindow,
                WindowsAndMessaging::{
                    ClipCursor, GetClientRect, GetClipCursor, GetSystemMetrics, GetWindowPlacement,
                    GetWindowRect, IsIconic, ShowCursor, IDC_APPSTARTING, IDC_ARROW, IDC_CROSS,
                    IDC_HAND, IDC_HELP, IDC_IBEAM, IDC_NO, IDC_SIZEALL, IDC_SIZENESW, IDC_SIZENS,
                    IDC_SIZENWSE, IDC_SIZEWE, IDC_WAIT, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN,
                    SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SW_MAXIMIZE, WINDOWPLACEMENT,
                },
            },
        },
    };

    pub fn encode_wide(string: impl AsRef<OsStr>) -> Vec<u16> {
        string.as_ref().encode_wide().chain(once(0)).collect()
    }

    pub fn decode_wide(mut wide_c_string: &[u16]) -> OsString {
        if let Some(null_pos) = wide_c_string.iter().position(|c| *c == 0) {
            wide_c_string = &wide_c_string[..null_pos];
        }

        OsString::from_wide(wide_c_string)
    }

    pub fn has_flag<T>(bitset: T, flag: T) -> bool
    where
        T: Copy + PartialEq + BitAnd<T, Output = T>,
    {
        bitset & flag == flag
    }

    pub(crate) fn win_to_err(result: BOOL) -> Result<(), io::Error> {
        if result != false.into() {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub enum WindowArea {
        Outer,
        Inner,
    }

    impl WindowArea {
        pub fn get_rect(self, hwnd: HWND) -> Result<RECT, io::Error> {
            let mut rect = unsafe { mem::zeroed() };

            match self {
                WindowArea::Outer => {
                    win_to_err(unsafe { GetWindowRect(hwnd, &mut rect) })?;
                }
                WindowArea::Inner => unsafe {
                    let mut top_left = mem::zeroed();

                    win_to_err(ClientToScreen(hwnd, &mut top_left))?;
                    win_to_err(GetClientRect(hwnd, &mut rect))?;
                    rect.left += top_left.x;
                    rect.top += top_left.y;
                    rect.right += top_left.x;
                    rect.bottom += top_left.y;
                },
            }

            Ok(rect)
        }
    }

    pub fn is_maximized(window: HWND) -> bool {
        unsafe {
            let mut placement: WINDOWPLACEMENT = mem::zeroed();
            placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
            GetWindowPlacement(window, &mut placement);
            placement.showCmd == SW_MAXIMIZE
        }
    }

    pub fn set_cursor_hidden(hidden: bool) {
        static HIDDEN: AtomicBool = AtomicBool::new(false);
        let changed = HIDDEN.swap(hidden, Ordering::SeqCst) ^ hidden;
        if changed {
            unsafe { ShowCursor(BOOL::from(!hidden)) };
        }
    }

    pub fn get_cursor_clip() -> Result<RECT, io::Error> {
        unsafe {
            let mut rect: RECT = mem::zeroed();
            win_to_err(GetClipCursor(&mut rect)).map(|_| rect)
        }
    }

    /// Sets the cursor's clip rect.
    ///
    /// Note that calling this will automatically dispatch a `WM_MOUSEMOVE` event.
    pub fn set_cursor_clip(rect: Option<RECT>) -> Result<(), io::Error> {
        unsafe {
            let rect_ptr = rect
                .as_ref()
                .map(|r| r as *const RECT)
                .unwrap_or(ptr::null());
            win_to_err(ClipCursor(rect_ptr))
        }
    }

    pub fn get_desktop_rect() -> RECT {
        unsafe {
            let left = GetSystemMetrics(SM_XVIRTUALSCREEN);
            let top = GetSystemMetrics(SM_YVIRTUALSCREEN);
            RECT {
                left,
                top,
                right: left + GetSystemMetrics(SM_CXVIRTUALSCREEN),
                bottom: top + GetSystemMetrics(SM_CYVIRTUALSCREEN),
            }
        }
    }

    pub fn is_focused(window: HWND) -> bool {
        window == unsafe { GetActiveWindow() }
    }

    pub fn is_minimized(window: HWND) -> bool {
        unsafe { IsIconic(window) != false.into() }
    }

    pub fn get_instance_handle() -> HMODULE {
        // Gets the instance handle by taking the address of the
        // pseudo-variable created by the microsoft linker:
        // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483

        // This is preferred over GetModuleHandle(NULL) because it also works in DLLs:
        // https://stackoverflow.com/questions/21718027/getmodulehandlenull-vs-hinstance

        extern "C" {
            static __ImageBase: IMAGE_DOS_HEADER;
        }

        unsafe { &__ImageBase as *const _ as _ }
    }

    // Helper function to dynamically load function pointer.
    // `library` and `function` must be zero-terminated.
    pub(super) fn get_function_impl(library: &str, function: &str) -> Option<*const c_void> {
        assert_eq!(library.chars().last(), Some('\0'));
        assert_eq!(function.chars().last(), Some('\0'));

        // Library names we will use are ASCII so we can use the A version to avoid string conversion.
        let module = unsafe { LoadLibraryA(library.as_ptr()) };
        if module == 0 {
            return None;
        }

        unsafe { GetProcAddress(module, function.as_ptr()) }.map(|function_ptr| function_ptr as _)
    }

    macro_rules! get_function {
        ($lib:expr, $func:ident) => {
            $crate::dark_mode::util::get_function_impl(
                concat!($lib, '\0'),
                concat!(stringify!($func), '\0'),
            )
            .map(|f| unsafe { std::mem::transmute::<*const _, $func>(f) })
        };
    }

    pub type SetProcessDPIAware = unsafe extern "system" fn() -> BOOL;
    pub type SetProcessDpiAwareness =
        unsafe extern "system" fn(value: PROCESS_DPI_AWARENESS) -> HRESULT;
    pub type SetProcessDpiAwarenessContext =
        unsafe extern "system" fn(value: DPI_AWARENESS_CONTEXT) -> BOOL;
    pub type GetDpiForWindow = unsafe extern "system" fn(hwnd: HWND) -> u32;
    pub type GetDpiForMonitor = unsafe extern "system" fn(
        hmonitor: HMONITOR,
        dpi_type: MONITOR_DPI_TYPE,
        dpi_x: *mut u32,
        dpi_y: *mut u32,
    ) -> HRESULT;
    pub type EnableNonClientDpiScaling = unsafe extern "system" fn(hwnd: HWND) -> BOOL;
    pub type AdjustWindowRectExForDpi = unsafe extern "system" fn(
        rect: *mut RECT,
        dwStyle: u32,
        bMenu: BOOL,
        dwExStyle: u32,
        dpi: u32,
    ) -> BOOL;

    pub static GET_DPI_FOR_WINDOW: Lazy<Option<GetDpiForWindow>> =
        Lazy::new(|| get_function!("user32.dll", GetDpiForWindow));
    pub static ADJUST_WINDOW_RECT_EX_FOR_DPI: Lazy<Option<AdjustWindowRectExForDpi>> =
        Lazy::new(|| get_function!("user32.dll", AdjustWindowRectExForDpi));
    pub static GET_DPI_FOR_MONITOR: Lazy<Option<GetDpiForMonitor>> =
        Lazy::new(|| get_function!("shcore.dll", GetDpiForMonitor));
    pub static ENABLE_NON_CLIENT_DPI_SCALING: Lazy<Option<EnableNonClientDpiScaling>> =
        Lazy::new(|| get_function!("user32.dll", EnableNonClientDpiScaling));
    pub static SET_PROCESS_DPI_AWARENESS_CONTEXT: Lazy<Option<SetProcessDpiAwarenessContext>> =
        Lazy::new(|| get_function!("user32.dll", SetProcessDpiAwarenessContext));
    pub static SET_PROCESS_DPI_AWARENESS: Lazy<Option<SetProcessDpiAwareness>> =
        Lazy::new(|| get_function!("shcore.dll", SetProcessDpiAwareness));
    pub static SET_PROCESS_DPI_AWARE: Lazy<Option<SetProcessDPIAware>> =
        Lazy::new(|| get_function!("user32.dll", SetProcessDPIAware));
}

static WIN10_BUILD_VERSION: Lazy<Option<u32>> = Lazy::new(|| {
    type RtlGetVersion = unsafe extern "system" fn(*mut OSVERSIONINFOW) -> NTSTATUS;
    let handle = get_function!("ntdll.dll", RtlGetVersion);

    if let Some(rtl_get_version) = handle {
        unsafe {
            let mut vi = OSVERSIONINFOW {
                dwOSVersionInfoSize: 0,
                dwMajorVersion: 0,
                dwMinorVersion: 0,
                dwBuildNumber: 0,
                dwPlatformId: 0,
                szCSDVersion: [0; 128],
            };

            let status = (rtl_get_version)(&mut vi);

            if status >= 0 && vi.dwMajorVersion == 10 && vi.dwMinorVersion == 0 {
                Some(vi.dwBuildNumber)
            } else {
                None
            }
        }
    } else {
        None
    }
});

static DARK_MODE_SUPPORTED: Lazy<bool> = Lazy::new(|| {
    // We won't try to do anything for windows versions < 17763
    // (Windows 10 October 2018 update)
    match *WIN10_BUILD_VERSION {
        Some(v) => v >= 17763,
        None => false,
    }
});

static DARK_THEME_NAME: Lazy<Vec<u16>> = Lazy::new(|| util::encode_wide("DarkMode_Explorer"));
static LIGHT_THEME_NAME: Lazy<Vec<u16>> = Lazy::new(|| util::encode_wide(""));

/// Attempt to set a theme on a window, if necessary.
/// Returns the theme that was picked
pub fn try_theme(hwnd: HWND) {
    if *DARK_MODE_SUPPORTED {
        let is_dark_mode = should_use_dark_mode();

        let theme_name = DARK_THEME_NAME.as_ptr();

        let status = unsafe { SetWindowTheme(hwnd, theme_name, ptr::null()) };

        if status == S_OK && set_dark_mode_for_window(hwnd, is_dark_mode) {
            println!("INFO: DARK MODE!");
        }
    }
}

fn set_dark_mode_for_window(hwnd: HWND, is_dark_mode: bool) -> bool {
    // Uses Windows undocumented API SetWindowCompositionAttribute,
    // as seen in win32-darkmode example linked at top of file.

    type SetWindowCompositionAttribute =
        unsafe extern "system" fn(HWND, *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;

    #[allow(clippy::upper_case_acronyms)]
    type WINDOWCOMPOSITIONATTRIB = u32;
    const WCA_USEDARKMODECOLORS: WINDOWCOMPOSITIONATTRIB = 26;

    #[allow(non_snake_case)]
    #[allow(clippy::upper_case_acronyms)]
    #[repr(C)]
    struct WINDOWCOMPOSITIONATTRIBDATA {
        Attrib: WINDOWCOMPOSITIONATTRIB,
        pvData: *mut c_void,
        cbData: usize,
    }

    static SET_WINDOW_COMPOSITION_ATTRIBUTE: Lazy<Option<SetWindowCompositionAttribute>> =
        Lazy::new(|| get_function!("user32.dll", SetWindowCompositionAttribute));

    if let Some(set_window_composition_attribute) = *SET_WINDOW_COMPOSITION_ATTRIBUTE {
        unsafe {
            // SetWindowCompositionAttribute needs a bigbool (i32), not bool.
            let mut is_dark_mode_bigbool = BOOL::from(is_dark_mode);

            let mut data = WINDOWCOMPOSITIONATTRIBDATA {
                Attrib: WCA_USEDARKMODECOLORS,
                pvData: &mut is_dark_mode_bigbool as *mut _ as _,
                cbData: std::mem::size_of_val(&is_dark_mode_bigbool) as _,
            };

            let status = set_window_composition_attribute(hwnd, &mut data);

            status != false.into()
        }
    } else {
        false
    }
}

fn should_use_dark_mode() -> bool {
    should_apps_use_dark_mode() && !is_high_contrast()
}

fn should_apps_use_dark_mode() -> bool {
    type ShouldAppsUseDarkMode = unsafe extern "system" fn() -> bool;
    static SHOULD_APPS_USE_DARK_MODE: Lazy<Option<ShouldAppsUseDarkMode>> = Lazy::new(|| unsafe {
        const UXTHEME_SHOULDAPPSUSEDARKMODE_ORDINAL: PCSTR = 132 as PCSTR;

        let module = LoadLibraryA("uxtheme.dll\0".as_ptr());

        if module == 0 {
            return None;
        }

        let handle = GetProcAddress(module, UXTHEME_SHOULDAPPSUSEDARKMODE_ORDINAL);

        handle.map(|handle| std::mem::transmute(handle))
    });

    SHOULD_APPS_USE_DARK_MODE
        .map(|should_apps_use_dark_mode| unsafe { (should_apps_use_dark_mode)() })
        .unwrap_or(false)
}

fn is_high_contrast() -> bool {
    let mut hc = HIGHCONTRASTA {
        cbSize: 0,
        dwFlags: 0,
        lpszDefaultScheme: ptr::null_mut(),
    };

    let ok = unsafe {
        SystemParametersInfoA(
            SPI_GETHIGHCONTRAST,
            std::mem::size_of_val(&hc) as _,
            &mut hc as *mut _ as _,
            0,
        )
    };

    ok != false.into() && util::has_flag(hc.dwFlags, HCF_HIGHCONTRASTON)
}
