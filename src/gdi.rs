use crate::*;

pub const SRCCOPY: u32 = 0x00CC0020;
pub const DEFAULT_CHARSET: DWORD = 1;
pub const OUT_OUTLINE_PRECIS: DWORD = 8;
pub const CLIP_DEFAULT_PRECIS: DWORD = 0;
pub const CLEARTYPE_QUALITY: DWORD = 5;
pub const TRANSPARENT: i32 = 1;
pub const RGN_AND: i32 = 1;

#[link(name = "Gdi32")]
extern "system" {
    pub fn StretchDIBits(
        hdc: *mut c_void,
        XDest: i32,
        YDest: i32,
        nDestWidth: i32,
        nDestHeight: i32,
        XSrc: i32,
        YSrc: i32,
        nSrcWidth: i32,
        nSrcHeight: i32,
        lpBits: *const c_void,
        lpBitsInfo: *const BITMAPINFO,
        iUsage: UINT,
        dwRop: DWORD,
    ) -> i32;
    pub fn CreateFontA(
        cHeight: i32,
        cWidth: i32,
        cEscapement: i32,
        cOrientation: i32,
        cWeight: i32,
        bItalic: DWORD,
        bUnderline: DWORD,
        bStrikeOut: DWORD,
        iCharSet: DWORD,
        iOutPrecision: DWORD,
        iClipPrecision: DWORD,
        iQuality: DWORD,
        iPitchAndFamily: DWORD,
        pszFaceName: LPCSTR,
    ) -> *mut c_void;
    pub fn TextOutA(hdc: *mut c_void, x: i32, y: i32, lpString: LPCSTR, c: i32) -> i32;
    pub fn SetTextColor(hdc: *mut c_void, color: u32) -> u32;
    pub fn SetBkMode(hdc: *mut c_void, mode: i32) -> i32;
    pub fn CreateCompatibleDC(hdc: *mut c_void) -> *mut c_void;
    pub fn DeleteDC(hdc: *mut c_void) -> i32;
    pub fn SelectObject(hdc: *mut c_void, h: *mut c_void) -> *mut c_void;
    pub fn SetRect(lprc: *mut RECT, xLeft: i32, yTop: i32, xRight: i32, yBottom: i32) -> BOOL;
    pub fn BeginPath(hdc: *mut c_void) -> i32;
    pub fn EndPath(hdc: *mut c_void) -> i32;
    pub fn SelectClipPath(hdc: *mut c_void, mode: i32) -> BOOL;
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BITMAPINFOHEADER {
    pub size: DWORD,
    pub width: LONG,
    pub height: LONG,
    pub planes: WORD,
    pub bit_count: WORD,
    pub compression: DWORD,
    pub size_image: DWORD,
    pub x_pels_per_meter: LONG,
    pub y_pels_per_meter: LONG,
    pub clr_used: DWORD,
    pub clr_important: DWORD,
}

impl BITMAPINFOHEADER {
    #[inline]
    pub const fn new(width: i32, height: i32) -> BITMAPINFO {
        BITMAPINFO {
            header: BITMAPINFOHEADER {
                size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                width,
                height: -height,
                planes: 1,
                bit_count: 32,
                compression: 0,
                size_image: 0,
                x_pels_per_meter: 0,
                y_pels_per_meter: 0,
                clr_used: 0,
                clr_important: 0,
            },
            colors: [RGBQUAD {
                blue: 0,
                green: 0,
                red: 0,
                reserved: 0,
            }],
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BITMAPINFO {
    pub header: BITMAPINFOHEADER,
    pub colors: [RGBQUAD; 1],
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct RGBQUAD {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}
