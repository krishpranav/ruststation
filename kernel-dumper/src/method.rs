use bitflags::bitflags;
use core::ffi::{c_int, CStr};
use core::num::NonZeroI32;
use korbis::file::OwnedFile;

pub trait DumpMethod: Sized {
    fn open(&elf, path: &CStr, flags: OpenFlags, mode: c_int) -> Result<OwnedFile<Self>, NonZeroI32>;

    fn write(&self, fd: c_int, buf: *const u8, len: usize) -> Result<usize, NonZeroI32>;
    fn fsync(&self, fd: c_int) -> Result<(), NonZeroI32>;
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct OpenFlags: u32 {}
}

pub struct OwnedFd<'a T: DumpMethod> {
    method: &'a T,
    fd: c_int,
}