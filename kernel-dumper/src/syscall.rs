use crate::method::{DumpMethod, OpenFlags, OwnedFd};
use core::ffi::{c_int, c_void, CStr};
use core::mem::transmute;
use core::num::NonZeroI32;
use korbis::thread::Thread;
use korbis::Kernel;
use x86_64::registers::control::Cr0;

pub struct SyscallMethod<K: Kernel> {
    sysents: &'static [Sysent<K>; 678],
}

impl<K: Kernel> SyscallMethod<K> {
    pub unsafe fn new(kernel: &K) -> Self {
        // Remove address checking from copyin, copyout and copyinstr.
        let base = kernel.elf().as_ptr();
        let cr0 = Cr0::read_raw();

        unsafe { Cr0::write_raw(cr0 & !(1 << 16)) };
        unsafe { Self::patch_kernel(base.cast_mut()) };
        unsafe { Cr0::write_raw(cr0) };

        Self {
            #[cfg(fw = "1100")]
            sysents: transmute(base.add(0x1101760)),
        }
    }

    #[cfg(fw = "1100")]
    unsafe fn patch_kernel(base: *mut u8) {
        let patches = [
            (0x2DDF42usize, [0x90u8; 2].as_slice()),
            (0x2DDF4E, &[0x90; 3]),
            (0x2DE037, &[0x90; 2]),
            (0x2DE043, &[0x90; 3]),
            (0x2DE4E3, &[0x90; 2]),
            (0x2DE4EF, &[0x90; 3]),
            (0x2DE520, &[0x90; 2]),
        ];

        for (off, patch) in patches {
            base.add(off)
                .copy_from_nonoverlapping(patch.as_ptr(), patch.len());
        }
    }
}

impl<K: Kernel> DumpMethod for SyscallMethod<K> {
    fn open(
        &self,
        path: &CStr,
        flags: OpenFlags,
        mode: c_int,
    ) -> Result<OwnedFd<Self>, NonZeroI32> {
        let td = Thread::current();
        let args = [path.as_ptr() as usize, flags.bits() as usize, mode as usize];

        let handler = self.sysents[5].handler;
        let errno = unsafe { handler(td, args.as_ptr().cast()) };

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(OwnedFd::new(self, unsafe {
                (*td).ret(0).try_into().unwrap()
            })),
        }
    }

    fn write(&self, fd: c_int, buf: *const u8, len: usize) -> Result<usize, NonZeroI32> {
        let td = Thread::current();
        let args = [fd as usize, buf as usize, len];

        let handler = self.sysents[4].handler;
        let errno = unsafe { handler(td, args.as_ptr().cast()) };

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(unsafe { (*td).ret(0) }),
        }
    }

    fn fsync(&self, fd: c_int) -> Result<(), NonZeroI32> {
        let td = Thread::current();
        let args = [fd as usize];

        let handler = self.sysents[95].handler;
        let errno = unsafe { handler(td, args.as_ptr().cast()) };

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(()),
        }
    }

    fn close(&self, fd: c_int) -> Result<(), NonZeroI32> {
        let td = Thread::current();
        let args = [fd as usize];

        let handler = self.sysents[6].handler;
        let errno = unsafe { handler(td, args.as_ptr().cast()) };

        match NonZeroI32::new(errno) {
            Some(v) => Err(v),
            None => Ok(()),
        }
    }
}

#[repr(C)]
struct Sysent<K: Kernel> {
    narg: c_int,
    handler: unsafe extern "C" fn(td: *mut K::Thread, uap: *const c_void) -> c_int,
    pad: [u8; 0x20],
}