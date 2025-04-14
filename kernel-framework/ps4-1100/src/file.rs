use core::sync::atomic::AtomicU32;

#[repr(C)]
pub struct File {
    pad: [u8; 0x28],
    refcnt: AtomicU32,
}

impl rsf::file::File for File {
    fn refcnt(&self) -> &AtomicU32 {
        todo!()
    }
}