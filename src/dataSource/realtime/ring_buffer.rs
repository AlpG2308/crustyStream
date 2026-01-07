use std::sync::atomic::{AtomicUsize,Ordering};
use std::mem::MaybeUninit;//invariant ininatilization
//
pub struct RingBuffer<T,const N: usize>{
    buffer:[<MaybeUninit<T>;N], const N: usize>,
    head: AtomicUsize,
    tail: AtomicUsize,
}
impl <T,const N: usize> RingBuffer {
    pub const fn new() -> Self{
        
    }
}


