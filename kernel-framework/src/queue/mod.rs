use core::marker::PhantomPinned;

#[repr(C)]
pub struct TailQueue<T> {
    pub first: *mut T,
    pub last: *mut *mut T,
    pub pin: PhantomPinned,
}

#[repr(C)]
pub struct TailQueueEntry<T> {
    pub next: *mut T,
    pub prev: *mut *mut T,
    pub pin: PhantomPinned,
}