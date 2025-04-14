use crate::Kernel;
use crate::lock::Mtx;
use crate::vnode::Vnode;
use core::ffi::{c_char, c_int};
use core::mem::MaybeUninit;
use core::num::NonZero;
use rsf::queue::TailQueueEntry;

#[repr(C)]
pub struct Mount {
    mtx: Mtx,
    pad1: [u8; 0x8],
    entry: TailQueueEntry<Self>,
    ops: *const FsOps,
    fs: *mut Filesystem,
    pad3: [u8; 0x38],
    flags: u64,
    pad4: [u8; 0x20],
    stats: FsStats,
}

impl rsf::mount::Mount<Kernel> for Mount {
    fn mtx(&self) -> *mut Mtx {
        &self.mtx as *const Mtx as *mut Mtx
    }

    unsafe fn entry(&self) -> &TailQueueEntry<Self> {
        &self.entry
    }

    unsafe fn entry_mut(&mut self) -> &mut TailQueueEntry<Self> {
        &mut self.entry
    }

    fn fs(&self) -> *mut Filesystem {
        self.fs
    }

    fn ops(&self) -> &'static FsOps {
        unsafe { &*self.ops }
    }

    unsafe fn flags(&self) -> u64 {
        self.flags
    }

    fn stats(&self) -> *mut FsStats {
        &self.stats as *const FsStats as *mut FsStats
    }
}

#[repr(C)]
pub struct Filesystem {
    pad1: [u8; 4],
    name: [c_char; 16],
}

impl rsf::mount::Filesystem for Filesystem {
    fn name(&self) -> *const c_char {
        self.name.as_ptr()
    }
}

#[repr(C)]
pub struct FsOps {
    pad1: [u8; 0x18],
    root: unsafe extern "C" fn(*mut Mount, c_int, *mut *mut Vnode) -> c_int,
}

impl rsf::mount::FsOps<Kernel> for FsOps {
    unsafe fn root(&self, mp: *mut Mount, flags: c_int) -> Result<*mut Vnode, NonZero<c_int>> {
        let mut vp = MaybeUninit::uninit();
        let errno = unsafe { (self.root)(mp, flags, vp.as_mut_ptr()) };

        match NonZero::new(errno) {
            Some(v) => Err(v),
            None => Ok(unsafe { vp.assume_init() }),
        }
    }
}

#[repr(C)]
pub struct FsStats {
    pad1: [u8; 0x128],
    mounted_from: [c_char; 88],
    pad2: [u8; 0x58],
}

impl rsf::mount::FsStats for FsStats {
    fn mounted_from(&self) -> *const c_char {
        self.mounted_from.as_ptr()
    }
}