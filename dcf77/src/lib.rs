#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

mod cycles_computer;
mod second_sync;

pub use crate::cycles_computer::CyclesComputer;
use crate::second_sync::{Edge, SecondSync};

#[derive(Debug)]
pub enum DecoderError {
    WrongTransition,
}

pub struct SignalSmoother<const X: usize> {
    buf: [bool; X],
    last_val: bool,
}

impl<const X: usize> SignalSmoother<X> {
    pub fn new() -> Self {
        Self {
            buf: [true; X],
            last_val: true,
        }
    }
    pub fn add_signal(&mut self, sig: bool) -> bool {
        self.buf.rotate_left(1);
        self.buf[X - 1] = sig;
        if self.buf.iter().all(|x| *x != self.last_val) {
            self.last_val = !self.last_val;
        }
        self.last_val
    }
}

pub struct DCF77Decoder {
    last_high_to_low: u32,
    last_low_to_high: u32,
    second_sync: SecondSync,
}

impl DCF77Decoder {
    pub fn new(cycles_computer: CyclesComputer) -> Self {
        Self {
            last_high_to_low: 0,
            last_low_to_high: 0,
            second_sync: SecondSync::new(cycles_computer),
        }
    }

    pub fn register_transition(&mut self, low_to_high: bool, now: u32) -> bool {
        if low_to_high {
            self.last_low_to_high = now;
            self.second_sync.register_transition(Edge::Rising, now);
        } else {
            self.last_high_to_low = now;
            self.second_sync.register_transition(Edge::Falling, now);
        }

        false
    }
}
