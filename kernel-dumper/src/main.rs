#![no_std]
#![no_main]

use crate::method::{DumpMethod, OpenFlags};
use core::arch::global_asm;
use core::cmp::min;
use core::ffi::c_int;
use core::mem::{size_of_val, zeroed};
use core::panic::PanicInfo;
use korbis::Kernel;
use x86_64::registers::model_specific::LStar;

#[cfg(method = "direct")]
mod direct;
mod method;
#[cfg(method = "syscall")]
mod syscall;

global_asm!(
    ".globl _start",
    ".section .text.entry",
    "_start:",
    "lea rdi, [rip]",
    "sub rdi, 7",
    "mov rax, rdi",
    "add rax, 0x80",
    "xor r8, r8",
    "0:",
    "mov rsi, [rax]",
    "mov rcx, [rax+8]",
    "add rax, 16",
    "test rsi, rsi",
    "jz 1f",
    "cmp rsi, 7",
    "jz 2f",
    "cmp rsi, 8",
    "jz 3f",
    "jmp 0b",
    "2:",
    "mov rdx, rdi",
    "add rdx, rcx",
    "jmp 0b",
    "3:",
    "mov r8, rcx",
    "jmp 0b",
    "1:",
    "test r8, r8",
    "jz main",
    "mov rsi, [rdx]",
    "mov rax, [rdx+8]",
    "mov rcx, [rdx+16]",
    "add rdx, 24",
    "sub r8, 24",
    "test eax, eax",
    "jz main",
    "cmp eax, 8",
    "jnz 1b",
    "add rsi, rdi",
    "add rcx, rdi",
    "mov [rsi], rcx",
    "jmp 1b",
);

#[no_mangle]
pub extern "C" fn main(_: *const u8) {
    let aslr = LStar::read() - 0xffffffff822001c0;
    let base = aslr + 0xffffffff82200000;
    let kernel = unsafe { init(base.as_ptr()) };

    #[cfg(method = "syscall")]
    let method = unsafe { crate::syscall::SyscallMethod::new(&kernel) };
    #[cfg(method = "direct")]
    let method = crate::direct::DirectMethod::new(kernel);

    let out = match method.open(
        c"/mnt/usb0/kernel.elf",
        OpenFlags::O_WRONLY | OpenFlags::O_CREAT | OpenFlags::O_TRUNC,
        0o777,
    ) {
        Ok(v) => v,
        Err(_) => {
            notify(&method, "Failed to open /mnt/usb0/kernel.elf");
            return;
        }
    };

    let mut data = unsafe { kernel.elf() };

    while !data.is_empty() {
        let fd = out.as_raw_fd();
        let len = min(data.len(), 0x4000);
        let buf = &data[..len];
        let written = match method.write(fd, buf.as_ptr(), buf.len()) {
            Ok(v) => v,
            Err(_) => {
                notify(&method, "Failed to write /mnt/usb0/kernel.elf");
                return;
            }
        };

        if written == 0 {
            notify(&method, "Not enough space to dump the kernel");
            return;
        }

        data = &data[written..];
    }

    if method.fsync(out.as_raw_fd()).is_err() {
        notify(
            &method,
            "Failed to synchronize changes to a /mnt/usb0/kernel.elf",
        );

        return;
    }

    notify(&method, "Dump completed!");
}

fn notify(method: &impl DumpMethod, msg: impl AsRef<[u8]>) {
    let devs = [c"/dev/notification0", c"/dev/notification1"];
    let mut fd = None;

    for dev in devs {
        if let Ok(v) = method.open(dev, OpenFlags::O_WRONLY, 0) {
            fd = Some(v);
            break;
        }
    }

    let fd = match fd {
        Some(v) => v,
        None => return,
    };

    let mut data: OrbisNotificationRequest = unsafe { zeroed() };
    let msg = msg.as_ref();
    let len = min(data.message.len() - 1, msg.len());

    data.target_id = -1;
    data.use_icon_image_uri = 1;
    data.message[..len].copy_from_slice(&msg[..len]);

    method
        .write(
            fd.as_raw_fd(),
            &data as *const OrbisNotificationRequest as _,
            size_of_val(&data),
        )
        .ok();
}

#[cfg(fw = "1100")]
unsafe fn init(base: *const u8) -> impl Kernel {
    korbis_1100::Kernel::new(base)
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
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