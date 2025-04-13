use core::ffi::c_char;

pub trait FsStats: Sized {
    fn mounted_from(&self) -> *const c_char;
}