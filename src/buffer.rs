use crate::traits::Buffer;
use std::array;

pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

//------ impl default for RingBuffer for implementation of Buffer T trait-----//
impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        assert!(N > 0, "Buffer capacity needs to be bigger than 0");
        Self {
            data: array::from_fn(|_| None),
            head: 0,
            tail: 0,
            len: 0,
        }
    }
}
//------ implement traits for RingBuffer------//
impl<T, const N: usize> Buffer<T> for RingBuffer<T, N> {
    //----capacity-----
    #[inline]
    fn capacity(&self) -> usize {
        N
    }
    //----len------
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
    //----push------
    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }
        self.data[self.head] = Some(value);
        // set new head index
        self.head = (self.head + 1) % N;
        self.len += 1;
        Ok(())
    }
    //----pop------
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.data[self.tail].take(); //take value and leave none
        //in place
        self.tail = (self.tail + 1) % N;
        self.len -= 1;
        value
    }
    // is full is empty not needed since both depend on len and capacity
}
//-------RingBufferIterator-------//
//Conceptually we define the struct -> Data and implement an iterator for that struct -> zero cost
//abstraction and monomorphism
//implement Buffer iterator and read only ReadBuffer iter struct
//iterator has lifetime 'a just like buffer
//Construct RingBufferIter struct for RingBuffer -> attr buffer, index, remaining
//Implement Iterator Trait for RingBufferIter structwith type Item and next function.
//----RingBufferIter struct
pub struct RingBufferIter<'a, T, const N: usize> {
    buffer: &'a RingBuffer<T, N>,
    index: usize,
    remaining: usize,
}
//------Iterator Constructor
impl<T, const N: usize> RingBuffer<T, N> {
    //iterator with lifetime decided by compiler
    pub fn iter(&self) -> RingBufferIter<'_, T, N> {
        //poping of tail index == tail
        //remaining is given by None since we are using take which replaces vals with None
        RingBufferIter {
            buffer: self,
            index: self.tail,
            remaining: self.len,
        }
    }
}
// implement Iterator
impl<'a, T, const N: usize> Iterator for RingBufferIter<'a, T, N> {
    //Rust hast Iterator trait already we want to tie it to our struct
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let item = self.buffer.data[self.index].as_ref();
        self.index = (self.index + 1) % N; //update index
        self.remaining -= 1; //reduce remaining by one we can use same approach as in RB but not
        //necessary here
        item
    }
}
//------WindowIter-------//
//Construct Window on Ring Buffer
//construct iterator
//iter
pub struct WindowIter<'a, T, const N: usize, const W: usize> {
    buffer: &'a RingBuffer<T, N>,
    index: usize,
    remaining: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn windows<const W: usize>(&self) -> WindowIter<'_, T, N, W> {
        assert!(W > 0, "Window size muste be larger than 0");
        assert!(W <= self.len, "Window larger than Buffer");
        //init window struct after asertion
        WindowIter {
            buffer: self,
            index: self.tail,
            remaining: self.len - W + 1,
        }
    }
}
impl<'a, T, const N: usize, const W: usize> Iterator for WindowIter<'a, T, N, W> {
    type Item = [&'a T; W];
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        //create closure over window
        let window = std::array::from_fn(|i| {
            let idx = (self.index + i) % N;
            self.buffer.data[idx].as_ref().expect("Invariant violated")
        });
        self.index = (self.index + 1) % N;
        self.remaining -= 1;
        Some(window)
    }
}
