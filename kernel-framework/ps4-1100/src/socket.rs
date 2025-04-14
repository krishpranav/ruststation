use core::ffi::{c_short, c_ushort};
use core::sync::atomic::{AtomicU16, Ordering};

#[repr(C)]
pub struct Socket {
    pad1: [u8; 0x6d],
    timeout: c_short,
    error: AtomicU16,
}

impl rsf::socket::Socket for Socket {
    fn error(&self) -> c_ushort {
        self.error.load(Ordering::Relaxed)
    }

    fn set_error(&self, v: c_ushort) {
        todo!()
    }

    fn timeout(&self) -> *mut c_short {
        todo!()
    }
}
