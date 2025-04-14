#[repr(C)]
pub struct Thread {
    pad: [u8; 0x398],
    ret: [usize; 2],
}

impl korbis::thread::Thread for Thread {
    fn ret(&self, i: usize) -> usize {
        self.ret[i]
    }
}