pub mod constants;

pub use constants::*;

pub type HWND = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type DWORD = u32;
pub type BOOL = i32;
pub type UINT = u32;
pub type LPCSTR = *const i8;
pub type LPCWSTR = *const u16;

use std::{
    ffi::{c_void, OsString},
    os::windows::prelude::OsStrExt,
};

//This type doesn't make any sense.
pub enum VOID {}

#[rustfmt::skip]
pub type WNDPROC = Option<unsafe extern "system" fn(param0: isize, param1: u32, param2: usize, param3: isize) -> isize>;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

#[link(name = "user32")]
#[link(name = "uxtheme")]
extern "system" {
    pub fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16;

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

    pub fn DefWindowProcA(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize;

    pub fn DispatchMessageA(lpMsg: *const MSG) -> isize;

    pub fn GetMessageA(
        lpMsg: *const MSG,
        hWnd: isize,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;

    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;

    pub fn GetLastError() -> u32;

    //Dark mode
    pub fn GetProcAddress(hModule: *mut VOID, lpProcName: *const i8) -> *mut VOID;
    pub fn LoadLibraryA(lpFileName: *const i8) -> *mut VOID;
    pub fn SetWindowTheme(
        hwnd: isize,
        pszSubAppName: *const u16,
        pszSubIdList: *const u16,
    ) -> isize;

    pub fn GetWindow(hWnd: isize, uCmd: u32) -> isize;
    pub fn GetForegroundWindow() -> isize;
}

///The window has a thin-line border
pub const WS_BORDER: u32 = 0x00800000;

///The window has a title bar (includes the WS_BORDER style).
pub const WS_CAPTION: u32 = 0x00C00000;

///The window is a child window. A window with this style cannot have a menu bar. This style cannot be used with the WS_POPUP style.
pub const WS_CHILD: u32 = 0x40000000;

///Same as the WS_CHILD style. __(DO NOT USE)__
pub const WS_CHILDWINDOW: u32 = WS_CHILD;

///Excludes the area occupied by child windows when drawing occurs within the parent window. This style is used when creating the parent window.
pub const WS_CLIPCHILDREN: u32 = 0x02000000;

///Clips child windows relative to each other; that is, when a particular child window receives a WM_PAINT message, the WS_CLIPSIBLINGS style clips all other overlapping child windows out of the region of the child window to be updated.
///
///If WS_CLIPSIBLINGS is not specified and child windows overlap, it is possible, when drawing within the client area of a child window, to draw within the client area of a neighboring child window.
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;

///The window is initially disabled. A disabled window cannot receive input from the user. To change this after a window has been created, use the EnableWindow function.
pub const WS_DISABLED: u32 = 0x08000000;

///The window has a border of a style typically used with dialog boxes. A window with this style cannot have a title bar.
pub const WS_DLGFRAME: u32 = 0x00400000;

///The window is the first control of a group of controls. The group consists of this first control and all controls defined after it, up to the next control with the WS_GROUP style.
///
///The first control in each group usually has the WS_TABSTOP style so that the user can move from group to group. The user can subsequently change the keyboard focus from one control in the group to the next control in the group by using the direction keys.
///You can turn this style on and off to change dialog box navigation. To change this style after a window has been created, use the SetWindowLong function.
pub const WS_GROUP: u32 = 0x00020000;

///The window has a horizontal scroll bar.
pub const WS_HSCROLL: u32 = 0x00100000;

///The window is initially minimized. Same as the WS_MINIMIZE style.
pub const WS_ICONIC: u32 = WS_MINIMIZE;

///The window is initially maximized.
pub const WS_MAXIMIZE: u32 = 0x01000000;

///The window has a maximize button. Cannot be combined with the WS_EX_CONTEXTHELP style. The WS_SYSMENU style must also be specified.
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;

///The window is initially minimized. Same as the WS_ICONIC style.
pub const WS_MINIMIZE: u32 = 0x20000000;

///The window has a minimize button. Cannot be combined with the WS_EX_CONTEXTHELP style. The WS_SYSMENU style must also be specified.
pub const WS_MINIMIZEBOX: u32 = 0x00020000;

///The window is an overlapped window. An overlapped window has a title bar and a border. Same as the WS_TILED style.
pub const WS_OVERLAPPED: u32 = 0x00000000;

///The window is an overlapped window. Same as the WS_TILEDWINDOW style.
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

///The window is a pop-up window. This style cannot be used with the WS_CHILD style.
pub const WS_POPUP: u32 = 0x80000000;

///The window is a pop-up window. The WS_CAPTION and WS_POPUPWINDOW styles must be combined to make the window menu visible.
pub const WS_POPUPWINDOW: u32 = WS_POPUP | WS_BORDER | WS_SYSMENU;

///The window has a sizing border. Same as the WS_THICKFRAME style.
pub const WS_SIZEBOX: u32 = WS_THICKFRAME;

///The window has a window menu on its title bar. The WS_CAPTION style must also be specified.
pub const WS_SYSMENU: u32 = 0x00080000;

///The window is a control that can receive the keyboard focus when the user presses the TAB key. Pressing the TAB key changes the keyboard focus to the next control with the WS_TABSTOP style.
///
///You can turn this style on and off to change dialog box navigation. To change this style after a window has been created, use the SetWindowLong function. For user-created windows and modeless dialogs to work with tab stops, alter the message loop to call the IsDialogMessage function.
pub const WS_TABSTOP: u32 = 0x00010000;

///The window has a sizing border. Same as the WS_SIZEBOX style.
pub const WS_THICKFRAME: u32 = 0x00040000;

///The window is an overlapped window. An overlapped window has a title bar and a border. Same as the WS_OVERLAPPED style.
pub const WS_TILED: u32 = WS_OVERLAPPED;

///The window is an overlapped window. Same as the WS_OVERLAPPEDWINDOW style.
pub const WS_TILEDWINDOW: u32 = WS_OVERLAPPEDWINDOW;

///The window is initially visible.
//This style can be turned on and off by using the ShowWindow or SetWindowPos function.
pub const WS_VISIBLE: u32 = 0x10000000;

///The window has a vertical scroll bar.
pub const WS_VSCROLL: u32 = 0x00200000;

pub const WS_EX_DLGMODALFRAME: u32 = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY: u32 = 0x00000004;
pub const WS_EX_TOPMOST: u32 = 0x00000008;
pub const WS_EX_ACCEPTFILES: u32 = 0x00000010;
pub const WS_EX_TRANSPARENT: u32 = 0x00000020;
pub const WS_EX_MDICHILD: u32 = 0x00000040;
pub const WS_EX_TOOLWINDOW: u32 = 0x00000080;
pub const WS_EX_WINDOWEDGE: u32 = 0x00000100;
pub const WS_EX_CLIENTEDGE: u32 = 0x00000200;
pub const WS_EX_CONTEXTHELP: u32 = 0x00000400;
pub const WS_EX_RIGHT: u32 = 0x00001000;
pub const WS_EX_LEFT: u32 = 0x00000000;
pub const WS_EX_RTLREADING: u32 = 0x00002000;
pub const WS_EX_LTRREADING: u32 = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: u32 = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR: u32 = 0x00000000;
pub const WS_EX_CONTROLPARENT: u32 = 0x00010000;
pub const WS_EX_STATICEDGE: u32 = 0x00020000;
pub const WS_EX_APPWINDOW: u32 = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW: u32 = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: u32 = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_LAYERED: u32 = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT: u32 = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP: u32 = 0x00200000;
pub const WS_EX_LAYOUTRTL: u32 = 0x00400000;
pub const WS_EX_COMPOSITED: u32 = 0x02000000;
pub const WS_EX_NOACTIVATE: u32 = 0x08000000;

/// Aligns the window's client area on a byte boundary (in the x direction). This style affects the width of the window and its horizontal placement on the display.
pub const CS_BYTEALIGNCLIENT: u32 = 0x1000;

/// Aligns the window on a byte boundary (in the x direction). This style affects the width of the window and its horizontal placement on the display.
pub const CS_BYTEALIGNWINDOW: u32 = 0x2000;

/// Allocates one device context to be shared by all windows in the class. Because window classes are process specific, it is possible for multiple threads of an application to create a window of the same class. It is also possible for the threads to attempt to use the device context simultaneously. When this happens, the system allows only one thread to successfully finish its drawing operation.
pub const CS_CLASSDC: u32 = 0x0040;

/// Sends a double-click message to the window procedure when the user double-clicks the mouse while the cursor is within a window belonging to the class.
pub const CS_DBLCLKS: u32 = 0x0008;

/// Enables the drop shadow effect on a window. The effect is turned on and off through SPI_SETDROPSHADOW. Typically, this is enabled for small, short-lived windows such as menus to emphasize their Z-order relationship to other windows. Windows created from a class with this style must be top-level windows; they may not be child windows.
pub const CS_DROPSHADOW: u32 = 0x00020000;

/// Indicates that the window class is an application global class. For more information, see the "Application Global Classes" section of About Window Classes.
pub const CS_GLOBALCLASS: u32 = 0x4000;

/// Redraws the entire window if a movement or size adjustment changes the width of the client area.
pub const CS_HREDRAW: u32 = 0x0002;

/// Disables Close on the window menu.
pub const CS_NOCLOSE: u32 = 0x0200;

/// Allocates a unique device context for each window in the class.
pub const CS_OWNDC: u32 = 0x0020;

/// Sets the clipping rectangle of the child window to that of the parent window so that the child can draw on the parent. A window with the CS_PARENTDC style bit receives a regular device context from the system's cache of device contexts. It does not give the child the parent's device context or device context settings. Specifying CS_PARENTDC enhances an application's performance.
pub const CS_PARENTDC: u32 = 0x0080;

/// Saves, as a bitmap, the portion of the screen image obscured by a window of this class. When the window is removed, the system uses the saved bitmap to restore the screen image, including other windows that were obscured. Therefore, the system does not send WM_PAINT messages to windows that were obscured if the memory used by the bitmap has not been discarded and if other screen actions have not invalidated the stored image. This style is useful for small windows (for example, menus or dialog boxes) that are displayed briefly and then removed before other screen activity takes place. This style increases the time required to display the window because the system must first allocate memory to store the bitmap.
pub const CS_SAVEBITS: u32 = 0x0800;

/// Redraws the entire window if a movement or size adjustment changes the height of the client area.
pub const CS_VREDRAW: u32 = 0x0001;

//
//
//
pub const CW_USEDEFAULT: i32 = -2147483648i32;
//
//
//

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WNDCLASSA {
    pub style: u32,
    pub wnd_proc: WNDPROC,
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

unsafe extern "system" fn test_proc(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> isize {
    match msg {
        // WM_CLOSE => drop(DestroyWindow(hWnd)),
        // WM_DESTROY => PostQuitMessage(0),
        WM_CLOSE => todo!(),
        WM_CREATE => {
            set_dark_mode(hwnd);
        }
        _ => return DefWindowProcA(hwnd, msg, wparam, lparam),
    }
    0
}

//TODO: https://devblogs.microsoft.com/oldnewthing/20100412-00/?p=14353
pub fn create_window(title: &str, width: i32, height: i32, options: u32) {
    //Title must be null terminated.
    let title = std::ffi::CString::new(title).unwrap();
    let wnd_class = WNDCLASSA {
        // wnd_proc: Some(DefWindowProcA),
        wnd_proc: Some(test_proc),
        class_name: title.as_ptr() as *const u8,
        style: CS_HREDRAW,
        ..Default::default()
    };

    let _result = unsafe { RegisterClassA(&wnd_class) };

    let hinstance = get_instance_handle();

    let hwnd = unsafe {
        CreateWindowExA(
            0,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            options,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            0,
            0,
            hinstance,
            std::ptr::null(),
        )
    };

    assert_ne!(hwnd, 0);

    // unsafe { set_dark_mode(hwnd) };
    // dark_mode::try_theme(hwnd);
    // let r = set_dark_mode_for_window(hwnd, false);
    // dbg!(r);
    // let str: Vec<u16> = std::ffi::OsString::from("DarkMode_Explorer")
    //     .encode_wide()
    //     .collect();
    // dbg!(unsafe { SetWindowTheme(hwnd, str.as_ptr(), 0 as *const u16) });
    // let mut is_dark_mode_bigbool = true as i32;
    // let mut data = WINDOWCOMPOSITIONATTRIBDATA {
    //     Attrib: WCA_USEDARKMODECOLORS,
    //     pvData: &mut is_dark_mode_bigbool as *mut _ as _,
    //     cbData: std::mem::size_of_val(&is_dark_mode_bigbool) as _,
    // };
}

pub fn window_event(msg: &mut MSG) {
    let message_result = unsafe { GetMessageA(msg, 0, 0, 0) };
    if message_result == 0 {
        // break;
    } else if message_result == -1 {
        let last_error = unsafe { GetLastError() };
        panic!("Error with `GetMessageA`, error code: {}", last_error);
    } else {
        unsafe {
            TranslateMessage(msg);
            DispatchMessageA(msg);
        }
    }
}

const WCA_USEDARKMODECOLORS: u32 = 26;

// pub type HMODULE = isize;
pub fn get_instance_handle() -> isize {
    // Gets the instance handle by taking the address of the
    // pseudo-variable created by the microsoft linker:
    // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483

    // This is preferred over GetModuleHandle(NULL) because it also works in DLLs:
    // https://stackoverflow.com/questions/21718027/getmodulehandlenull-vs-hinstance
    #[repr(C, packed(2))]
    pub struct IMAGE_DOS_HEADER {
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
        static __ImageBase: IMAGE_DOS_HEADER;
    }

    unsafe { &__ImageBase as *const _ as _ }
}

unsafe fn set_dark_mode(hwnd: isize) {
    #[repr(C)]
    pub struct WINDOWCOMPOSITIONATTRIBDATA {
        attrib: u32,
        data: *mut c_void,
        size: usize,
    }

    unsafe fn proc(module: *mut VOID, name: &str) -> *const c_void {
        GetProcAddress(module, name.as_ptr() as *const i8) as *const c_void
    }

    // let result = SetWindowTheme(hwnd, utf16!("DarkMode_Explorer").as_ptr(), std::ptr::null());
    // assert_eq!(result, 0);

    let user32 = LoadLibraryA(b"user32.dll\0" as *const u8 as *const i8);
    let uxtheme = LoadLibraryA(b"uxtheme.dll\0" as *const u8 as *const i8);

    let set_window = proc(user32, "SetWindowCompositionAttribute\0");
    let set_window = std::mem::transmute::<
        *const _,
        unsafe extern "system" fn(isize, *mut WINDOWCOMPOSITIONATTRIBDATA) -> i32,
    >(set_window);

    // let should = GetProcAddress(uxtheme, 106 as *const i8) as *const c_void;
    // let should = std::mem::transmute::<*const _, unsafe extern "system" fn() -> bool>(should);

    // let refresh = GetProcAddress(uxtheme, 104 as *const i8) as *const c_void;
    // let refresh = std::mem::transmute::<*const _, unsafe extern "system" fn()>(refresh);

    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WCA_USEDARKMODECOLORS,
        data: std::mem::transmute(&mut 1i32),
        size: std::mem::size_of::<i32>(),
    };

    let result = set_window(hwnd, &mut data);
    assert_ne!(result, 0);
}
