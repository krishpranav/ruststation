pub use self::fs::*;
pub use self::stats::*;
use crate::Kernel;
use crate::queue::TailQueueEntry;

mod fs;
mod stats;

pub trait Mount<K: Kernel>: Sized {
    fn mtx(&self) -> *mut K::Mtx;

    unsafe fn entry(&self) -> &TailQueueEntry<Self>;

    unsafe fn entry_mut(&mut self) -> &mut TailQueueEntry<Self>;

    fn fs(&self) -> *mut K::Filesystem;

    fn ops(&self) -> &'static K::FsOps;

    unsafe fn flags(&self) -> u64;

    fn stats(&self) -> *mut K::FsStats;
}