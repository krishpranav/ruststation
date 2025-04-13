use crate::Kernel;
use crate::fd::{AT_FDCWD, OpenFlags, openat, write_all};
use crate::pcpu::Pcpu;
use crate::uio::UioSeg;
use core::ffi::c_int;
use core::fmt::Write;

pub struct Notification {
    data: OrbisNotificationRequest,
    next: usize,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            data: OrbisNotificationRequest {
            }
        }
    }

    #[inline(never)]
    pub fn send<K: Kernel>(self, k: K) {
        let devs = [c"/dev/notifications0", c"/dev/notifications1"];
        let mut fd = None;

        for dev in devs.into_iter().map(|v| v.as_ptr()) {
            if let Ok(v) = unsafe { }
        }

        let fd = match fd {
            Some(v) => v,
            None => return,
        };

    }

}

#[repr(C)]
struct OrbisNotificationRequest {
    ty: c_int,
    req_id: c_int,
    priority: c_int,
    msg_id: c_int,
    target_id: c_int,
    user_id: c_int,
    unk1: c_int,
    unk2: c_int,
    app_id: c_int,
    error_num: c_int,
    unk3: c_int,
    use_icon_image_uri: u8,
    message: [u8; 1024],
    icon_uri: [u8; 1024],
    unk: [u8; 1024],
}