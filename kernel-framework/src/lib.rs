#![no_std]

use self::fd::OpenFlags;
use self::file::File;
use self::lock::{LockObject, Mtx};
use self::malloc::{Malloc, MallocFlags};
use self::mount::{Filesystem, FsOps, FsStats, Mount};
use self::namei::ComponentName;
use self::pcpu::Pcpu;
use self::queue::TailQueue;
use self::socket::{SockAddr, Socket};
use self::thread::Thread;
use self::ucred::Ucred;
use self::uio::{Uio, UioSeg};
use self::vnode::{Vnode, VnodeOp, VopLookup, VopRead, VopReadDir, VopUnlock, VopVector};
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_char, c_int};
use core::marker::PhantomData;
use core::mem::transmute;
use core::num::NonZero;
use core::ops::Deref;
use core::ptr::{null_mut, read_unaligned, write_unaligned};
pub use rs_macros::*;

pub mod fd;
pub mod file;
pub mod lock;
pub mod malloc;
pub mod mount;
pub mod namei;
pub mod notification;
pub mod panic;
pub mod pcpu;
pub mod queue;
pub mod socket;
pub mod thread;
pub mod ucred;
pub mod uio;
pub mod vnode;
mod namei;

pub trait Kernel: MappedKernel {
    const ACCEPT_MTX: StaticMut<Self::Mtx>;
    const EINTR: NonZero<c_int>;
    const EIO: NonZero<c_int>;
    const LK_EXCLUSIVE: c_int;
    const LK_SHARED: c_int;
    const LOOKUP: u64;
    const M_TEMP: StaticMut<Self::Malloc>;
    const MBF_MNTLSTLOCK: c_int;
    const MBF_NOWAIT: c_int;
    const MNT_RDONLY: u64;
    const MOUNTLIST: StaticMut<TailQueue<Self::Mount>>;
    const MOUNTLIST_MTX: StaticMut<Self::Mtx>;
    const NOCPU: u32;
    const PANIC: Function<unsafe extern "C" fn(*const c_char, ...) -> !>;
    const VDIR: c_int;
    const VOP_LOOKUP: StaticMut<Self::VnodeOp>;
    const VOP_READ: StaticMut<Self::VnodeOp>;
    const VOP_READDIR: StaticMut<Self::VnodeOp>;
    const VOP_UNLOCK: StaticMut<Self::VnodeOp>;
    const VREG: c_int;

    type ComponentName: ComponentName<Self>;
    type File: File;
    type Filesystem: Filesystem;
    type FsOps: FsOps<Self>;
    type FsStats: FsStats;
    type LockObject: LockObject;
    type Malloc: Malloc;
    type Mount: Mount<Self>;
    type Mtx: Mtx<Self>;
    type Pcpu: Pcpu<Self>;
    type Socket: Socket;
    type Thread: Thread<Self>;
    type Ucred: Ucred;
    type Uio: Uio<Self>;
    type Vnode: Vnode<Self>;
    type VnodeOp: VnodeOp;
    type VopLookup: VopLookup<Self>;
    type VopRead: VopRead<Self>;
    type VopReadDir: VopReadDir<Self>;
    type VopUnlock: VopUnlock;
    type VopVector: VopVector;

    fn get<O: Offset>(self, off: O) -> O::Ops {
        let addr = unsafe { self.addr().add(off.get()) };

        unsafe { <O::Ops as OffsetOps>::new(addr) }
    }

    unsafe fn fget(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        fp: *mut *mut Self::File,
        mode: c_int,
        maxprotp: *mut u8,
    ) -> c_int;

    unsafe fn fget_write(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        unused: c_int,
        fp: *mut *mut Self::File,
    ) -> c_int;

    unsafe fn fdrop(self, fp: *mut Self::File, td: *mut Self::Thread) -> c_int;

    unsafe fn free(self, addr: *mut u8, ty: *mut Self::Malloc);

    unsafe fn kern_openat(
        self,
        td: *mut Self::Thread,
        fd: c_int,
        path: *const c_char,
        seg: UioSeg,
        flags: OpenFlags,
        mode: c_int,
    ) -> c_int;

    unsafe fn kern_close(self, td: *mut Self::Thread, fd: c_int) -> c_int;

    unsafe fn kern_fsync(self, td: *mut Self::Thread, fd: c_int, fullsync: c_int) -> c_int;

    unsafe fn kern_writev(self, td: *mut Self::Thread, fd: c_int, auio: *mut Self::Uio) -> c_int;

    unsafe fn malloc(self, size: usize, ty: *mut Self::Malloc, flags: MallocFlags) -> *mut u8;

    unsafe fn mtx_lock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    unsafe fn mtx_unlock_flags(
        self,
        m: *mut Self::Mtx,
        opts: c_int,
        file: *const c_char,
        line: c_int,
    );

    unsafe fn sleep(
        self,
        ident: *mut (),
        lock: *mut Self::LockObject,
        priority: c_int,
        wmesg: *const c_char,
        timo: c_int,
    ) -> c_int;

    unsafe fn soaccept(self, so: *mut Self::Socket, nam: *mut *mut SockAddr) -> c_int;

    unsafe fn sobind(
        self,
        so: *mut Self::Socket,
        nam: *mut SockAddr,
        td: *mut Self::Thread,
    ) -> c_int;

    unsafe fn soclose(self, so: *mut Self::Socket) -> c_int;

    unsafe fn socreate(
        self,
        dom: c_int,
        aso: *mut *mut Self::Socket,
        ty: c_int,
        proto: c_int,
        cred: *mut Self::Ucred,
        td: *mut Self::Thread,
    ) -> c_int;

    unsafe fn solisten(self, so: *mut Self::Socket, backlog: c_int, td: *mut Self::Thread)
                       -> c_int;

    unsafe fn strlen(self, s: *const c_char) -> usize;

    unsafe fn vfs_busy(self, mp: *mut Self::Mount, flags: c_int) -> c_int;

    unsafe fn vfs_unbusy(self, mp: *mut Self::Mount);

    unsafe fn vop_lookup(self, vec: *mut Self::VopVector, args: *mut Self::VopLookup) -> c_int;

    unsafe fn vop_read(self, vec: *mut Self::VopVector, args: *mut Self::VopRead) -> c_int;

    unsafe fn vop_readdir(self, vec: *mut Self::VopVector, args: *mut Self::VopReadDir) -> c_int;

    unsafe fn vop_unlock(self, vec: *mut Self::VopVector, args: *mut Self::VopUnlock) -> c_int;

    unsafe fn vput(self, vp: *mut Self::Vnode);
}

pub trait MappedKernel: Default + Sized + Copy + Send + Sync + 'static {
    fn addr(self) -> *const u8;
}

pub trait Offset: Copy {
    type Ops: OffsetOps;

    fn get(self) -> usize;
}

pub trait OffsetOps: Copy {
    unsafe fn new(addr: *const u8) -> Self;
}

pub struct Static<T> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T> Static<T> {
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Static<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Static<T> {}

impl<T> Offset for Static<T> {
    type Ops = ImmutableOps<T>;

    fn get(self) -> usize {
        self.off
    }
}

pub struct ImmutableOps<T>(*const T);

impl<T> OffsetOps for ImmutableOps<T> {
    unsafe fn new(addr: *const u8) -> Self {
        Self(addr.cast())
    }
}

impl<T> Clone for ImmutableOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ImmutableOps<T> {}

impl<T> Deref for ImmutableOps<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

pub struct StaticMut<T> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T> StaticMut<T> {
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for StaticMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for StaticMut<T> {}

impl<T> Offset for StaticMut<T> {
    type Ops = MutableOps<T>;

    fn get(self) -> usize {
        self.off
    }
}

pub struct MutableOps<T>(*mut T);

impl<T> MutableOps<T> {
    pub fn as_mut_ptr(self) -> *mut T {
        self.0
    }

    pub unsafe fn write(self, value: T) {
        unsafe { self.0.write(value) };
    }
}

impl<T: Copy> MutableOps<T> {
    pub unsafe fn read(self) -> T {
        unsafe { self.0.read() }
    }
}

impl<T> Clone for MutableOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for MutableOps<T> {}

impl<T> OffsetOps for MutableOps<T> {
    unsafe fn new(addr: *const u8) -> Self {
        Self(addr.cast_mut().cast())
    }
}

pub struct Function<T: KernelFn> {
    off: usize,
    phantom: PhantomData<T>,
}

impl<T: KernelFn> Function<T> {
    pub const unsafe fn new(off: usize) -> Self {
        Self {
            off,
            phantom: PhantomData,
        }
    }
}

impl<T: KernelFn> Clone for Function<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: KernelFn> Copy for Function<T> {}

impl<T: KernelFn> Offset for Function<T> {
    type Ops = FunctionOps<T>;

    fn get(self) -> usize {
        self.off
    }
}

pub struct FunctionOps<T> {
    addr: *const u8,
    phantom: PhantomData<T>,
}

impl<T: KernelFn> FunctionOps<T> {
    pub fn as_ptr(self) -> T {
        unsafe { T::from_addr(self.addr) }
    }
}

impl<T> Clone for FunctionOps<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for FunctionOps<T> {}

impl<T> OffsetOps for FunctionOps<T> {
    unsafe fn new(addr: *const u8) -> Self {
        Self {
            addr,
            phantom: PhantomData,
        }
    }
}

pub trait KernelFn: Copy {
    unsafe fn from_addr(addr: *const u8) -> Self;
}

impl<R, A1> KernelFn for unsafe extern "C" fn(A1, ...) -> R {
    unsafe fn from_addr(addr: *const u8) -> Self {
        unsafe { transmute(addr) }
    }
}

pub struct Allocator<K: Kernel>(PhantomData<K>);

impl<K: Kernel> Allocator<K> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    #[inline(never)]
    unsafe fn alloc(layout: Layout, flags: MallocFlags) -> *mut u8 {
        let size = if layout.align() <= 8 {
            layout.size()
        } else {
            match layout.size().checked_add(layout.align() - 8) {
                Some(v) => v,
                None => return null_mut(),
            }
        };

        let size = match size.checked_add(size_of::<usize>()) {
            Some(v) => v,
            None => return null_mut(),
        };

        let k = K::default();
        let t = k.get(K::M_TEMP);
        let mem = unsafe { k.malloc(size, t.as_mut_ptr(), flags) };

        if mem.is_null() {
            return null_mut();
        }

        let misaligned = (mem as usize) % layout.align();
        let adjust = if misaligned == 0 {
            0
        } else {
            layout.align() - misaligned
        };

        let mem = unsafe { mem.add(adjust) };

        unsafe { write_unaligned(mem.add(layout.size()).cast(), adjust) };

        mem
    }
}

impl<K: Kernel> Default for Allocator<K> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<K: Kernel> GlobalAlloc for Allocator<K> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { Self::alloc(layout, MallocFlags::WAITOK) }
    }

    #[inline(never)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let adjusted: usize = unsafe { read_unaligned(ptr.add(layout.size()).cast()) };
        let ptr = unsafe { ptr.sub(adjusted) };

        let k = K::default();
        let t = k.get(K::M_TEMP);

        unsafe { k.free(ptr, t.as_mut_ptr()) };
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { Self::alloc(layout, MallocFlags::WAITOK | MallocFlags::ZERO) }
    }
}