use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering}; //invariant ininatilization
pub struct RingBuffer<T, const N: usize> {
    buffer: [MaybeUninit<T>; N], // zero cost abstraction -> avoid memory overhead by
    // ininatilization
    head: AtomicUsize,
    tail: AtomicUsize,
}
impl<T, const N: usize> RingBuffer<T, N> {
    //---------Constructor----------//
    pub const fn new() -> Self {
        // init a datatype that tells compiler we dont have a valid T yet
        // but the MaybeUninit is a valid value that is initalized
        // essentially creating a place holder without using memory for it
        const UNINIT: MaybeUninit<()> = MaybeUninit::uninit();
        // now we have to exchange our place holder with the actual type an array of len N with
        // type T -> its an inherently unsafe operation so we tell the compiler no worries we will
        // init the actual data before using it we take responsibilty not the compiler
        let buffer = unsafe {
            std::mem::transmute::<[MaybeUninit<()>; N], [MaybeUninit<T>; N]>([UNINIT; N])
        };
        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }
    //-------Producer Push operation----------//
    //control ordering of data we want to push data while reading the data in a structured manner
    //push---pop
    //make sure that in a multi threaded program we dont get data races or undefined behaviors
    //tell compiler and hardware with release, acquire and relaxed how to acces data
    //lock free approach with acquire and release
    //acquire memory for critical section and release if critical part is over
    //use relaxed for parts which only need weak control
    //use the unsafe keyword because we are controlling the memory layout and pointer system
    //compiler cant check it but atomics are by design safe operations
    pub const fn push(&self, value: T) -> Result<(), T> {
        let head = self.head.load(Ordering::Relaxed); //relax Ordering -> only touch memory of that
        //value
        let next = (head + 1) % N;
        if next == self.tail.load(Ordering::Acquire) {
            return (Err(value)); // buffer full
        }
        unsafe {
            self.buffer[head].as_ptr().write(value); //write new value to head  
            //
        }
        self.head.store(next, Ordering::Release); //Add next value to head and order all values
        //before with acquire
        Ok(())
    }
    //------------Consumer-------------//
    pub const fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed); //relax Ordering -> only touch memory of that
        //value
        if tail == self.head.load(Ordering::Acquire) {
            return None; // buffer empty
        }
        let value = unsafe {
            self.buffer[tail].as_ptr().read(); //write new value to head  
            //
        };
        let next = (tail + 1) % N;
        self.tail.store(next, Ordering::Release); //Add next value to head and order all values
        //before with acquire
        Some(value)
    }
}
