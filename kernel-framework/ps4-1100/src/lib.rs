#![no_std]

use self::file::File;
use self::lock::{LockObject, Mtx};
use self::malloc::Malloc;
use self::mount::{Filesystem, FsOps, FsStats, Mount};
use self::namei::ComponentName;
use self::pcpu::Pcpu;
use self::socket::Socket;
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::Uio;
use self::vnode::{Vnode, VnodeOp, VopLookup, VopRead, VopReadDir, VopUnlock, VopVector};
use core::ffi::{c_char, c_int};
use core::num::NonZero;
use rsf::fd::OpenFlags;
use rsf::malloc::MallocFlags;
use rsf::queue::TailQueue;
use rsf::socket::SockAddr;
use rsf::uio::UioSeg;
use rsf::{Function, MappedKernel, StaticMut, offset};

mod file;
mod lock;
mod malloc;
mod mount;
mod namei;
mod pcpu;
mod socket;
mod thread;
mod ucred;
mod uio;
mod vnode;

#[derive(Clone, Copy, MappedKernel)]
pub struct Kernel(*const u8);

impl rsf::Kernel for Kernel {
    #[offset(0x221CCF8)]
    const ACCEPT_MTX: StaticMut<Self::Mtx>;
    const EINTR: NonZero<c_int> = unsafe { NonZero::new_unchecked(4) };
    const EIO: NonZero<c_int> = unsafe { NonZero::new_unchecked(5) };
    const LK_EXCLUSIVE: c_int = 0x80000;
    const LK_SHARED: c_int = 0x200000;
    const LOOKUP: u64 = 0;
    #[offset(0x15415B0)]
    const M_TEMP: StaticMut<Self::Malloc>;
    const MBF_MNTLSTLOCK: c_int = 2;
    const MBF_NOWAIT: c_int = 1;
    const MNT_RDONLY: u64 = 0x0000000000000001;
    #[offset(0x1A6AD60)]
    const MOUNTLIST: StaticMut<TailQueue<Self::Mount>>;
    #[offset(0x22D0F10)]
    const MOUNTLIST_MTX: StaticMut<Self::Mtx>;
    const NOCPU: u32 = 0xff;
    #[offset(0x1987C0)]
    const PANIC: Function<unsafe extern "C" fn(*const c_char, ...) -> !>;
    const VDIR: c_int = 2;
    #[offset(0x15308F0)]
    const VOP_LOOKUP: StaticMut<Self::VnodeOp>;
    #[offset(0x1531F70)]
    const VOP_READ: StaticMut<Self::VnodeOp>;
    #[offset(0x1533A00)]
    const VOP_READDIR: StaticMut<Self::VnodeOp>;
    #[offset(0x1534360)]
    const VOP_UNLOCK: StaticMut<Self::VnodeOp>;
    const VREG: c_int = 1;

    type ComponentName = ComponentName;
    type File = File;
    type Filesystem = Filesystem;
    type FsOps = FsOps;
    type FsStats = FsStats;
    type LockObject = LockObject;
    type Malloc = Malloc;
    type Mount = Mount;
    type Mtx = Mtx;
    type Pcpu = Pcpu;
    type Socket = Socket;
    type Thread = Thread;
    type Ucred = Ucred;
    type Uio = Uio;
    type Vnode = Vnode;
    type VnodeOp = VnodeOp;
    type VopLookup = VopLookup;
    type VopRead = VopRead;
    type VopReadDir = VopReadDir;
    type VopUnlock = VopUnlock;
    type VopVector = VopVector;

    #[offset(0x419040)]
    unsafe fn fget(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        fp: *mut *mut Self::File,
        mode: c_int,
        maxprotp: *mut u8,
    ) -> c_int;

    #[offset(0x4191C0)]
    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        unused: c_int,
        fp: *mut *mut Self::File,
    ) -> c_int;

    #[offset(0x4161B0)]
    unsafe fn fdrop(self, fp: *mut Self::File, td: *mut Self::Thread) -> c_int;

    #[offset(0x1A43E0)]
    unsafe fn free(self, addr: *mut u8, ty: *mut Self::Malloc);

    #[offset(0xE63B0)]
    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: OpenFlags,
        mode: c_int,
    ) -> c_int;

    #[offset(0x416920)]
    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    #[offset(0xEAD50)]
    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    #[offset(0xDD340)]
    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    #[offset(0x1A4220)]
    unsafe fn malloc(self, size: usize, ty: *mut Self::Malloc, flags: MallocFlags) -> *mut u8;

    #[offset(0x10E6A0)]
    unsafe fn mtx_lock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    #[offset(0x10E950)]
    unsafe fn mtx_unlock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    #[offset(0x365F50)]
    unsafe fn sleep(
        self,
        ident: *mut (),
        lock: *mut Self::LockObject,
        priority: c_int,
        wmesg: *const c_char,
        timo: c_int,
    ) -> c_int;

    #[offset(0x264AF0)]
    unsafe fn soaccept(self, so: *mut Self::Socket, nam: *mut *mut SockAddr) -> c_int;

    #[offset(0x264600)]
    unsafe fn sobind(
        self,
        so: *mut Self::Socket,
        nam: *mut SockAddr,
        td: *mut Self::Thread,
    ) -> c_int;

    #[offset(0x264680)]
    unsafe fn soclose(self, so: *mut Self::Socket) -> c_int;

    #[offset(0x263890)]
    unsafe fn socreate(
        self,
        dom: c_int,
        aso: *mut *mut Self::Socket,
        ty: c_int,
        proto: c_int,
        cred: *mut Self::Ucred,
        td: *mut Self::Thread,
    ) -> c_int;

    #[offset(0x264620)]
    unsafe fn solisten(self, so: *mut Self::Socket, backlog: c_int, td: *mut Self::Thread)
                       -> c_int;

    #[offset(0x21DC40)]
    unsafe fn strlen(self, s: *const c_char) -> usize;

    #[offset(0x37BAF0)]
    unsafe fn vfs_busy(self, mp: *mut Self::Mount, flags: c_int) -> c_int;

    #[offset(0x37BC60)]
    unsafe fn vfs_unbusy(self, mp: *mut Self::Mount);

    #[offset(0x12D870)]
    unsafe fn vop_lookup(self, vec: *mut Self::VopVector, args: *mut Self::VopLookup) -> c_int;

    #[offset(0x12E7E0)]
    unsafe fn vop_read(self, vec: *mut Self::VopVector, args: *mut Self::VopRead) -> c_int;

    #[offset(0x12FB00)]
    unsafe fn vop_readdir(self, vec: *mut Self::VopVector, args: *mut Self::VopReadDir) -> c_int;

    #[offset(0x1300A0)]
    unsafe fn vop_unlock(self, vec: *mut Self::VopVector, args: *mut Self::VopUnlock) -> c_int;

    #[offset(0x37E9B0)]
    unsafe fn vput(self, vp: *mut Self::Vnode);
}

unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}