//implement a buffer trait
//push
//pop
//len
//capacity
//is_empty
//is_full
pub trait Buffer<T> {
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn push(&mut self, value: T) -> Result<(), T>;
    fn pop(&mut self) -> Option<T>;
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
}
