use crate::{buffer::RingBuffer, traits::Buffer};

#[test]
fn push_pop_ring_buffer() {
    let mut ring_buff: RingBuffer<i32, 4> = RingBuffer::new();
    ring_buff.push(1).unwrap();
    ring_buff.push(2).unwrap();
    ring_buff.push(3).unwrap();

    assert_eq!(ring_buff.pop(), Some(1));
    assert_eq!(ring_buff.pop(), Some(2));
    assert_eq!(ring_buff.pop(), Some(3));
    assert_eq!(ring_buff.pop(), None);
}
#[test]
fn buffer_full() {
    let mut ring_buff: RingBuffer<i32, 2> = RingBuffer::new();
    assert!(ring_buff.push(1).is_ok());
    assert!(ring_buff.push(2).is_ok());
    assert!(ring_buff.push(3).is_err());
}
#[test]
fn wrap_around() {
    let mut ring_buff: RingBuffer<i32, 2> = RingBuffer::new();
    ring_buff.push(1).unwrap();
    ring_buff.push(2).unwrap();
    ring_buff.pop();
    ring_buff.push(3).unwrap();

    assert_eq!(ring_buff.pop(), Some(2));
    assert_eq!(ring_buff.pop(), Some(3));
}
#[test]
fn iterate_in_order() {
    let mut ring_buff: RingBuffer<i32, 3> = RingBuffer::new();
    ring_buff.push(10).unwrap();
    ring_buff.push(20).unwrap();
    ring_buff.push(30).unwrap();
    let collected: Vec<_> = ring_buff.iter().copied().collect();
    assert_eq!(collected, vec![10, 20, 30])
}
#[test]
fn simple_window_refs() {
    let mut ring_buff: RingBuffer<i32, 3> = RingBuffer::new();
    ring_buff.push(10).unwrap();
    ring_buff.push(20).unwrap();
    ring_buff.push(30).unwrap();
    ring_buff.push(40).unwrap();
    ring_buff.push(50).unwrap();
    ring_buff.push(60).unwrap();
    ring_buff.push(70).unwrap();
    ring_buff.push(80).unwrap();
    //ring_buff.push(90).unwrap();
    let mut idx = 0;
    let expected = vec![vec![10, 20, 30], vec![40, 50, 60], vec![70, 80, 10]];
    let mut win = ring_buff.windows::<3>();
    while let Some(w) = win.next() {
        for (i, v) in w.iter().enumerate() {
            assert_eq!(**v, expected[idx][i]);
        }
        idx += 1;
    }
    assert_eq!(idx, expected.len());
}
