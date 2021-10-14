#![cfg_attr(not(test), no_std)]
use core::ops::Index;

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
        if self.cpos < self.buf.len() - 1 {
            self.buf[self.cpos] = val;
            self.cpos = self.cpos + 1;
        } else {
            // rotate all elements in the buffer by one to the left
            for i in 1..self.buf.len() - 1 {
                self.buf[i - 1] = self.buf[i];
            }
            self.buf[self.cpos] = val;
        }
    }
}

impl<const X: usize> Index<usize> for RingBuffer<X> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}
