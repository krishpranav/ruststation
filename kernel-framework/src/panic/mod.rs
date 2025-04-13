use self::msg::Message;
use crate::Kernel;
use core::fmt::Write;
use core::panic::PanicInfo;

mod msg;

pub fn panic<K: Kernel>(k: K, i: &PanicInfo) -> ! {
    let mut m = Message::default();
    let _ = write!(m, "{i}");

    let f = k.get(K::PANIC).as_ptr();

    unsafe { f(c"%s".as_ptr(), m.as_ptr()) };
}