use crate::Kernel;
use crate::pcpu::Pcpu;
use crate::thread::Thread;
use crate::uio::{IoVec, Uio, UioSeg};
use bitflags::bitflags;
use core::ffi::{c_char, c_int};
use core::marker::PhantomData;
use core::num::NonZero;

pub const AT_FDCWD: c_int = -100;

#[inline(never)]
pub unsafe fn openat<K: Kernel>(
    kern: K,
    fd: c_int,
    path: *const c_char,
    seg: UioSeg,
    flags: OpenFlags,
    mode: c_int,
) -> Result<OwnedFd<K>, NonZero<c_int>> {
    let td = K::Pcpu::curthread();
    let errno = unsafe { kern.kern_openat(td, fd, path, seg, flags, mode) };

    match NonZero::new(errno) {
        Some(v) => Err(v),
        None => Ok(OwnedFd {
            kern,
            fd: unsafe { (*td).ret(0).try_into().unwrap() },
            phantom: PhantomData,
        }),
    }
}

#[inline(never)]
pub unsafe fn write_all<K: Kernel>(
    kern: K,
    fd: c_int,
    mut data: &[u8],
    td: *mut K::Thread,
) -> Result<(), NonZero<c_int>> {
    while !data.is_empty() {
        let written = match unsafe { write(kern, fd, data, td) } {
            Ok(v) => v,
            Err(e) if e == K::EINTR => continue,
            Err(e) => return Err(e),
        };

        if written == 0 {
            return Err(K::EIO);
        }

        data = &data[written..];
    }

    Ok(())
}

#[inline(never)]
pub unsafe fn write<K: Kernel>(
    kern: K,
    fd: c_int,
    data: &[u8],
    td: *mut K::Thread,
) -> Result<usize, NonZero<c_int>> {
    let mut vec = IoVec {
        ptr: data.as_ptr().cast_mut(),
        len: data.len(),
    };

    let mut uio = unsafe { K::Uio::write(&mut vec, td).unwrap() };
    let errno = unsafe { kern.kern_writev(td, fd, &mut uio) };

    match NonZero::new(errno) {
        Some(v) => Err(v),
        None => Ok(unsafe { (*td).ret(0) }),
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpenFlags: c_int {
        const O_RDONLY = 0x00000000;
        const O_WRONLY = 0x00000001;
        const O_RDWR = 0x00000002;
        const O_ACCMODE = Self::O_WRONLY.bits() | Self::O_RDWR.bits();
        const O_SHLOCK = 0x00000010;
        const O_EXLOCK = 0x00000020;
        const O_CREAT = 0x00000200;
        const O_TRUNC = 0x00000400;
        const O_EXCL = 0x00000800;
        const O_EXEC = 0x00040000;
        const O_CLOEXEC = 0x00100000;
    }
}

pub struct OwnedFd<K: Kernel> {
    kern: K,
    fd: c_int,
    phantom: PhantomData<*const ()>,
}

impl<K: Kernel> OwnedFd<K> {
    pub fn as_raw_fd(&self) -> c_int {
        self.fd
    }
}

impl<K: Kernel> Drop for OwnedFd<K> {
    #[inline(never)]
    fn drop(&mut self) {
        assert_eq!(
            unsafe { self.kern.kern_close(K::Pcpu::curthread(), self.fd) },
            0
        );
    }
}