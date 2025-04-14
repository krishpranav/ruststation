#![no_std]

use self::elf::ProgramType;
use self::file::{File, OwnedFile};
use self::thread::Thread;
use self::uio::{Uio, UioSeg};
use core::ffi::{c_char, c_int};
use core::num::NonZeroI32;
use core::ptr::null_mut;

pub use korbis_macros::*;

pub mod elf;
pub mod file;
pub mod thread;
pub mod uio;

pub trait Kernel: Copy + Send + Sync + 'static {
    type File: File;
    type Thread: Thread;
    type Uio: Uio<Self>;

    unsafe fn new(base: *const u8) -> Self;

    unsafe fn elf(self) -> &'static [u8];

    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        unused: c_int,
        fp: *mut *mut Self::File,
    ) -> c_int;

    unsafe fn fdrop(self, fp: *mut Self::File, td: *mut Self::Thread) -> c_int;

    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: c_int,
        mode: c_int,
    ) -> c_int;

    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    unsafe fn get_mapped_elf(base: *const u8) -> &'static [u8] {
        let e_phnum = base.add(0x38).cast::<u16>().read() as usize;
        let progs = core::slice::from_raw_parts(base.add(0x40), e_phnum * 0x38);
        let mut end = base as usize;

        for h in progs.chunks_exact(0x38) {
            let ty = ProgramType::new(u32::from_le_bytes(h[0x00..0x04].try_into().unwrap()));

            if !matches!(ty, ProgramType::PT_LOAD | ProgramType::PT_SCE_RELRO) {
                continue;
            }

            let addr = usize::from_le_bytes(h[0x10..0x18].try_into().unwrap());
            let len = usize::from_le_bytes(h[0x28..0x30].try_into().unwrap());
            let align = usize::from_le_bytes(h[0x30..0x38].try_into().unwrap());

            assert!(addr >= end);

            end = addr + len.next_multiple_of(align);
        }

        let len = end - (base as usize);

        core::slice::from_raw_parts(base, len)
    }
}

pub trait KernelExt: Kernel {
    fn fget_write(self, td: *mut Self::Thread, fd: c_int) -> Result<OwnedFile<Self>, NonZeroI32>;
}

impl<T: Kernel> KernelExt for T {
    fn fget_write(self, td: *mut Self::Thread, fd: c_int) -> Result<OwnedFile<Self>, NonZeroI32> {
        let mut fp = null_mut();
        let errno = unsafe { self.fget_write(td, fd, 0, &mut fp) };

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(unsafe { OwnedFile::new(self, fp) }),
        }
    }
}