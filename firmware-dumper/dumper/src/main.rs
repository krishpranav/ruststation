#![no_std]
#![no_main]

use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::arch::global_asm;
use core::cmp::min;
use core::ffi::{c_int, CStr};
use core::hint::unreachable_unchecked;
use core::mem::{zeroed, MaybeUninit};
use core::panic::PanicInfo;
use core::ptr::null_mut;
use rsfw::ps4::PartItem;
use rsfw::{DumpItem, MAGIC};
use rsf::fd::{openat, write_all, OpenFlags, AT_FDCWD};
use rsf::lock::MtxLock;
use rsf::mount::{Filesystem, FsOps, FsStats, Mount};
use rsf::namei::ComponentName;
use rsf::pcpu::Pcpu;
use rsf::thread::Thread;
use rsf::uio::{IoVec, Uio, UioSeg};
use rsf::vnode::{DirEnt, Vnode, VopLookup, VopRead, VopReadDir};
use rsf::{Allocator, Kernel};

extern crate alloc;

global_asm!(
    ".globl _start",
    ".section .text.startup",
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

fn run<K: Kernel>(k: K) {
    let path = c"/mnt/usb0/firmware.obf";
    let flags = OpenFlags::O_WRONLY | OpenFlags::O_CREAT | OpenFlags::O_TRUNC;
    let fd = match unsafe { openat(k, AT_FDCWD, path.as_ptr(), UioSeg::Kernel, flags, 0o777) } {
        Ok(v) => v,
        Err(_) => {
            notify(k, "Could not open dump file");
            return;
        }
    };

    let fd = fd.as_raw_fd();

    if !write_dump(k, fd, MAGIC) {
        return;
    }

    let mtx = k.var(K::MOUNTLIST_MTX);

    unsafe { k.mtx_lock_flags(mtx.ptr(), 0, c"".as_ptr(), 0) };

    let list = k.var(K::MOUNTLIST);
    let mut mp = unsafe { (*list.ptr()).first };
    let mut items = 0u32;
    let mut ok = true;

    while !mp.is_null() {
        unsafe { k.vfs_busy(mp, K::MBF_MNTLSTLOCK) };

        let lock = unsafe { MtxLock::new(k, (*mp).mtx()) };
        let r = if unsafe { (*mp).flags() & K::MNT_RDONLY != 0 } {
            unsafe { dump_mount(k, fd, mp, lock) }
        } else {
            drop(lock);
            Some(0)
        };

        ok = match r {
            Some(v) => {
                items += v;
                true
            }
            None => false,
        };

        unsafe { k.mtx_lock_flags(mtx.ptr(), 0, c"".as_ptr(), 0) };

        unsafe { k.vfs_unbusy(mp) };

        if !ok {
            break;
        }

        mp = unsafe { (*mp).entry().next };
    }

    unsafe { k.mtx_unlock_flags(mtx.ptr(), 0, c"".as_ptr(), 0) };

    if !ok {
        return;
    }

    if !write_dump(k, fd, &[DumpItem::End.into()]) || !write_dump(k, fd, &items.to_le_bytes()) {
        return;
    }

    let td = K::Pcpu::curthread();
    let errno = unsafe { k.kern_fsync(td, fd, 1) };

    if errno != 0 {
        notify(k, "Couldn't flush dump file");
        return;
    }

    notify(k, "Dump completed!");
}

unsafe fn dump_mount<K: Kernel>(
    k: K,
    fd: c_int,
    mp: *mut K::Mount,
    lock: MtxLock<K>,
) -> Option<u32> {
    drop(lock);

    let fs = (*mp).fs();
    let fs = CStr::from_ptr((*fs).name()).to_bytes();

    if !matches!(fs, b"exfatfs" | b"ufs") {
        return Some(0);
    }

    if !write_dump(k, fd, &[DumpItem::Ps4Part.into()]) {
        return None;
    }

    if !write_dump(k, fd, &[0]) {
        return None;
    }

    if !write_dump(k, fd, &fs.len().to_le_bytes()) || !write_dump(k, fd, fs) {
        return None;
    }

    let stats = (*mp).stats();
    let dev = CStr::from_ptr((*stats).mounted_from()).to_bytes();

    if !write_dump(k, fd, &dev.len().to_le_bytes()) || !write_dump(k, fd, dev) {
        return None;
    }

    let vp = match (*mp).ops().root(mp, K::LK_SHARED) {
        Ok(v) => v,
        Err(_) => {
            notify(k, "Couldn't get root vnode");
            return None;
        }
    };

    let mut items = 1;
    let mut pending = VecDeque::from([PendingVnode {
        k,
        vnode: vp,
        path: Vec::new(),
    }]);

    while let Some(p) = pending.pop_front() {
        let ty = (*p.vnode).ty();
        let ty = if ty == K::VDIR {
            PartItem::Directory
        } else if ty == K::VREG {
            PartItem::File
        } else {
            let m = format!("Unknown vnode {ty}");
            notify(k, &m);
            return None;
        };

        if !write_dump(k, fd, &[ty.into()]) {
            return None;
        }

        if !write_dump(k, fd, &p.path.len().to_le_bytes()) || !write_dump(k, fd, &p.path) {
            return None;
        }

        items += 1;

        let ok = match ty {
            PartItem::End => unreachable_unchecked(),
            PartItem::Directory => list_files(k, p, &mut pending),
            PartItem::File => dump_file(k, p, fd),
        };

        if !ok {
            return None;
        }
    }

    if write_dump(k, fd, &[PartItem::End.into()]) {
        Some(items)
    } else {
        None
    }
}

unsafe fn list_files<K: Kernel>(
    k: K,
    p: PendingVnode<K>,
    pending: &mut VecDeque<PendingVnode<K>>,
) -> bool {
    let td = K::Pcpu::curthread();
    let mut off = 0;

    loop {
        let mut buf = MaybeUninit::<DirEnt<256>>::uninit();
        let mut vec = IoVec {
            ptr: buf.as_mut_ptr().cast(),
            len: size_of_val(&buf),
        };

        let mut io = Uio::read(&mut vec, off, td).unwrap();
        let mut eof = MaybeUninit::uninit();
        let mut args = VopReadDir::new(
            k,
            p.vnode,
            &mut io,
            (*td).cred(),
            eof.as_mut_ptr(),
            null_mut(),
            null_mut(),
        );

        let errno = k.vop_readdir((*p.vnode).ops(), &mut args);

        if errno != 0 {
            notify(k, "Couldn't read directory entry");
            return false;
        }

        off = io.offset().try_into().unwrap();

        let len = size_of_val(&buf) - usize::try_from(io.remaining()).unwrap();
        let mut buf = core::slice::from_raw_parts_mut::<u8>(buf.as_mut_ptr().cast(), len);

        while !buf.is_empty() {
            let ent = buf.as_mut_ptr() as *mut DirEnt<1>;
            let len: usize = (*ent).len.into();

            buf = &mut buf[len..];

            let len = (*ent).name_len.into();
            let name = core::slice::from_raw_parts::<u8>((*ent).name.as_ptr().cast(), len);

            if matches!(name, b"." | b"..") {
                continue;
            }

            let mut path = p.path.clone();

            path.push(b'/');
            path.extend_from_slice(name);

            let mut child = MaybeUninit::uninit();
            let name = (*ent).name.as_mut_ptr();
            let mut cn = ComponentName::new(k, K::LOOKUP, K::LK_SHARED, name, td);
            let mut args = VopLookup::new(k, p.vnode, child.as_mut_ptr(), &mut cn);
            let errno = k.vop_lookup((*p.vnode).ops(), &mut args);

            if errno != 0 {
                notify(k, "Couldn't lookup child vnode");
                return false;
            }

            pending.push_back(PendingVnode {
                k,
                vnode: child.assume_init(),
                path,
            });
        }

        if eof.assume_init() != 0 {
            break;
        }
    }

    true
}

unsafe fn dump_file<K: Kernel>(k: K, p: PendingVnode<K>, fd: c_int) -> bool {
    if !write_dump(k, fd, &[0]) {
        return false;
    }

    let td = K::Pcpu::curthread();
    let mut buf = vec![0; 0xFFFF];
    let mut off = 0;

    loop {
        let mut vec = IoVec {
            ptr: buf.as_mut_ptr(),
            len: buf.len(),
        };

        let mut io = Uio::read(&mut vec, off, td).unwrap();
        let mut args = VopRead::new(k, p.vnode, &mut io, 0, (*td).cred());
        let errno = k.vop_read((*p.vnode).ops(), &mut args);

        if errno != 0 {
            notify(k, "Couldn't read a file");
            return false;
        }

        off = io.offset().try_into().unwrap();

        let len = buf.len() - usize::try_from(io.remaining()).unwrap();

        if len == 0 {
            break;
        }

        let buf = &buf[..len];
        let len: u16 = len.try_into().unwrap();

        if !write_dump(k, fd, &len.to_le_bytes()) || !write_dump(k, fd, buf) {
            return false;
        }
    }

    write_dump(k, fd, &0u16.to_le_bytes())
}

#[inline(never)]
fn write_dump<K: Kernel>(k: K, fd: c_int, data: &[u8]) -> bool {
    let td = K::Pcpu::curthread();

    match unsafe { write_all(k, fd, data, td) } {
        Ok(_) => true,
        Err(_) => {
            notify(k, "Couldn't write dump file");
            false
        }
    }
}

#[inline(never)]
fn notify<K: Kernel>(k: K, msg: &str) {
    let devs = [c"/dev/notification0", c"/dev/notification1"];
    let mut fd = None;

    for dev in devs.into_iter().map(|v| v.as_ptr()) {
        if let Ok(v) = unsafe { openat(k, AT_FDCWD, dev, UioSeg::Kernel, OpenFlags::O_WRONLY, 0) } {
            fd = Some(v);
            break;
        }
    }

    let fd = match fd {
        Some(v) => v,
        None => return,
    };

    let mut data: OrbisNotificationRequest = unsafe { zeroed() };
    let msg = msg.as_bytes();
    let len = min(data.message.len() - 1, msg.len());

    data.target_id = -1;
    data.use_icon_image_uri = 1;
    data.message[..len].copy_from_slice(&msg[..len]);

    let len = size_of_val(&data);
    let data = &data as *const OrbisNotificationRequest as *const u8;
    let data = unsafe { core::slice::from_raw_parts(data, len) };
    let td = K::Pcpu::curthread();

    unsafe { write_all(k, fd.as_raw_fd(), data, td).ok() };
}

#[allow(dead_code)]
#[cfg_attr(target_os = "none", panic_handler)]
fn panic(_: &PanicInfo) -> ! {
    unsafe { unreachable_unchecked() };
}

struct PendingVnode<K: Kernel> {
    k: K,
    vnode: *mut K::Vnode,
    path: Vec<u8>,
}

impl<K: Kernel> Drop for PendingVnode<K> {
    fn drop(&mut self) {
        unsafe { self.k.vput(self.vnode) };
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
