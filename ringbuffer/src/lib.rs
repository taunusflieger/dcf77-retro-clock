#![cfg_attr(not(test), no_std)]
use core::ops::Index;

#[derive(Debug)]
pub struct RingBuffer<const X: usize> {
    buf: [u32; X],
    cpos: usize,
}

impl<const X: usize> RingBuffer<X> {
    pub fn new() -> Self {
        Self {
            buf: [0; X],
            cpos: 0,
        }
    }

    pub fn push(&mut self, val: u32) {
        if self.cpos < self.buf.len() {
            self.buf[self.cpos] = val;
            self.cpos = self.cpos + 1;
        } else {
            // rotate all elements in the buffer by one to the left
            for i in 1..self.buf.len() {
                self.buf[i - 1] = self.buf[i];
            }
            self.buf[self.cpos - 1] = val;
        }
    }
}

impl<const X: usize> Index<usize> for RingBuffer<X> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_push_first() {
        let mut buf: RingBuffer<10>;

        buf = RingBuffer::new();

        buf.push(1);

        assert_eq!(buf[0], 1);
    }

    #[test]
    fn test_push_second() {
        let mut buf: RingBuffer<10>;

        buf = RingBuffer::new();

        buf.push(1);
        buf.push(2);

        assert_eq!(buf[1], 2);
    }

    #[test]
    fn test_buffer_wrap() {
        let mut buf: RingBuffer<10>;
        let mut i = 0;

        buf = RingBuffer::new();

        while i < 21 {
            buf.push(i);
            i = i + 1;
        }
        assert_eq!(buf[0], 11);
        assert_eq!(buf[9], 20);
    }
}
