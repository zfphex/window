use crate::*;

pub const SRCCOPY: u32 = 0x00CC0020;

#[link(name = "Gdi32")]
extern "system" {
    pub fn StretchDIBits(
        hdc: *mut VOID,
        XDest: i32,
        YDest: i32,
        nDestWidth: i32,
        nDestHeight: i32,
        XSrc: i32,
        YSrc: i32,
        nSrcWidth: i32,
        nSrcHeight: i32,
        lpBits: *const VOID,
        lpBitsInfo: *const BITMAPINFO,
        iUsage: UINT,
        dwRop: DWORD,
    ) -> i32;
}

#[inline(always)]
pub const fn create_bitmap(width: i32, height: i32) -> BITMAPINFO {
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
