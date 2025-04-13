use core::cmp::min;
use core::ffi::c_char;
use core::fmt::Write;

pub struct Message {
    buf: [u8; 2048],
    pos: usize,
}

impl Message {
    pub fn as_ptr(&self) -> *const c_char {
        self.buf.as_ptr().cast()
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            buf: [0; 2048],
            pos: 0,
        }
    }
}

impl Write for Message {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = min(s.len(), (self.buf.len() - 1) - self.pos);
        let buf = unsafe { self.buf.as_mut_ptr().add(self.pos) };

        unsafe { buf.copy_from_nonoverlapping(s.as_ptr(), len) };
        self.pos += len;

        Ok(())
    }
}