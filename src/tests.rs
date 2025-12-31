use crate::{buffer::RingBuffer, traits::Buffer};

#[test]
fn push_pop_ring_buffer() {
    let mut ringBuff: RingBuffer<i32, 4> = RingBuffer::new();
    ringBuff.push(1).unwrap();
    ringBuff.push(2).unwrap();
    ringBuff.push(3).unwrap();

    assert_eq!(ringBuff.pop(), Some(1));
    assert_eq!(ringBuff.pop(), Some(2));
    assert_eq!(ringBuff.pop(), Some(3));
    assert_eq!(ringBuff.pop(), None);
}
#[test]
fn buffer_full() {
    let mut ringBuff: RingBuffer<i32, 2> = RingBuffer::new();
    assert!(ringBuff.push(1).is_ok());
    assert!(ringBuff.push(2).is_ok());
    assert!(ringBuff.push(3).is_err());
}
#[test]
fn wrap_around() {
    let mut ringBuff: RingBuffer<i32, 2> = RingBuffer::new();
    ringBuff.push(1).unwrap();
    ringBuff.push(2).unwrap();
    ringBuff.pop();
    ringBuff.push(3).unwrap();

    assert_eq!(ringBuff.pop(), Some(2));
    assert_eq!(ringBuff.pop(), Some(3));
}
#[test]
fn iterate_in_order() {
    let mut ringBuff: RingBuffer<i32, 3> = RingBuffer::new();
    ringBuff.push(10).unwrap();
    ringBuff.push(20).unwrap();
    ringBuff.push(30).unwrap();
    let collected: Vec<_> = ringBuff.iter().copied().collect();
    assert_eq!(collected, vec![10, 20, 30])
}

#[test]
fn windows_basic() {
    let mut ringBuff: RingBuffer<i32, 5> = RingBuffer::new();

    ringBuff.push(1).unwrap();
    ringBuff.push(2).unwrap();
    ringBuff.push(3).unwrap();
    ringBuff.push(4).unwrap();

    let windows: Vec<Vec<i32>> = ringBuff.windows::<3>().map(|w| w.iter().copied()).collect();

    assert_eq!(windows, vec![vec![1, 2, 3], vec![2, 3, 4],]);
}
#[test]
fn windows_exact_fit() {
    let mut ringBuff: RingBuffer<i32, 4> = RingBuffer::new();

    ringBuff.push(10).unwrap();
    ringBuff.push(20).unwrap();
    ringBuff.push(30).unwrap();

    let windows: Vec<Vec<i32>> = ringBuff
        .windows::<4>()
        .map(|w| w.iter().copied().collect())
        .collect();
    assert_eq!(windows, vec![vec![10, 20, 30]]);
}
#[test]
fn windows_wraparound() {
    let mut ringBuff: RingBuffer<i32, 4> = RingBuffer::new();

    ringBuff.push(1).unwrap();
    ringBuff.push(2).unwrap();
    ringBuff.push(3).unwrap();
    ringBuff.pop(); // remove 1
    ringBuff.push(4).unwrap(); // wraparound happens here
    ringBuff.push(5).unwrap(); // now buffer contains [2,3,4,5]

    let windows: Vec<Vec<i32>> = ringBuff
        .windows::<2>()
        .map(|w| w.iter().copied().collect())
        .collect();

    assert_eq!(windows, vec![vec![2, 3], vec![3, 4], vec![4, 5],]);
}
#[test]
fn windows_size_one() {
    let mut ringBuff: RingBuffer<i32, 3> = RingBuffer::new();

    ringBuff.push(7).unwrap();
    ringBuff.push(8).unwrap();

    let windows: Vec<i32> = ringBuff.windows::<1>().map(|w| *w[0]).collect();

    assert_eq!(windows, vec![7, 8]);
}
#[test]
#[should_panic]
fn windows_too_large_panics() {
    let mut ringBuff: RingBuffer<i32, 3> = RingBuffer::new();
    ringBuff.push(1).unwrap();
    ringBuff.push(2).unwrap();

    let _ = ringBuff.windows::<3>(); // len = 2, W = 3
}
#[test]
fn windows_do_not_consume_buffer() {
    let mut ringBuff: RingBuffer<i32, 4> = RingBuffer::new();

    ringBuff.push(1).unwrap();
    ringBuff.push(2).unwrap();
    ringBuff.push(3).unwrap();

    let _: Vec<_> = ringBuff.windows::<2>().collect();

    assert_eq!(ringBuff.len(), 3);
    assert_eq!(ringBuff.pop(), Some(1));
}
