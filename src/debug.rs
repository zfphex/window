use crate::*;

#[link(name = "user32")]
unsafe extern "system" {
    pub fn VirtualQuery(
        lpAddress: *mut c_void,
        lpBuffer: *mut MEMORY_BASIC_INFORMATION,
        dwLength: usize,
    ) -> usize;

}
#[repr(C)]
#[derive(Debug)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut c_void,
    pub AllocationBase: *mut c_void,
    pub AllocationProtect: usize,
    pub RegionSize: usize,
    pub State: usize,
    pub Protect: usize,
    pub Type: usize,
}
