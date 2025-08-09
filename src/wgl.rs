use crate::*;

#[link(name = "Opengl32")]
extern "system" {
    pub fn wglCreateContext(hdc: *mut core::ffi::c_void) -> HGLRC;
    pub fn wglMakeCurrent(hdc: *mut core::ffi::c_void, hglrc: HGLRC) -> i32;
    pub fn wglDeleteContext(hglrc: HGLRC) -> i32;
}

#[derive(Debug, Default)]
pub struct WglContext {
    pub hglrc: HGLRC,
}

impl WglContext {
    #[inline]
    pub const fn is_valid(&self) -> bool {
        !self.hglrc.is_null()
    }
}
