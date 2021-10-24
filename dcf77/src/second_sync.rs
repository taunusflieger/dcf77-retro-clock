// Second Sync
// Let's say we want the system to be able to sync
// the second up to a signal/noise ratio of 1:99. This means we need to be able to
// handle 100 falling flanks per second. If we store the timestamp of the falling
// flanks in a list for 3 seconds, we need a list with 300 entries. After three
// seconds we look if we have at least two entries which have a timestamp
// difference of 1000ms => these should than be considered as candidates for a
// sync. We validate these candidates for another 3 seconds. If they are good, we
// have a sync of the second signal in under 10 seconds. If not, we start all
// over.
//
//
use crate::cycles_computer::CyclesComputer;
use ringbuffer::RingBuffer;
//use rtt_target::rprintln;

const BUFFER_SIZE: usize = 300;
const SIGNAL_SEARCH_WINDOW_LEN: u32 = 10;

pub enum Edge {
    Falling,
    Rising,
}

pub struct SecondSync {
    timestamps_edge_down: RingBuffer<BUFFER_SIZE>,
    timestamps_edge_up: RingBuffer<BUFFER_SIZE>,
    cycles_per_1000ms: u32,
    signal_search_window_ms: u32,
}

impl SecondSync {
    pub fn new(cycles_computer: CyclesComputer) -> Self {
        SecondSync {
            timestamps_edge_down: RingBuffer::new(),
            timestamps_edge_up: RingBuffer::new(),
            cycles_per_1000ms: cycles_computer.from_cycles(1000),
            signal_search_window_ms: cycles_computer.from_cycles(SIGNAL_SEARCH_WINDOW_LEN),
        }
    }

    pub fn register_transition(&mut self, signal: Edge, now: u32) -> bool {
        match signal {
            Edge::Falling => {
                self.timestamps_edge_down.push(now);
            }
            Edge::Rising => {
                self.timestamps_edge_up.push(now);
            }
        }
        self.check_second_sync()
    }

    // NOTE Current implementation does not handle overflow of CYCCT
    fn check_second_sync(&mut self) -> bool {
        let mut first_second_mark = 0;
        let mut second_second_mark = 0;
        // let mut third_second_mark = 0;
        for i in 0..BUFFER_SIZE - 1 {
            for j in (i + 1)..BUFFER_SIZE - 1 {
                let d = self.timestamps_edge_down[j] - self.timestamps_edge_up[i];
                if d > (self.cycles_per_1000ms - self.signal_search_window_ms / 2)
                    && d < (self.cycles_per_1000ms + self.signal_search_window_ms / 2)
                {
                    first_second_mark = self.timestamps_edge_down[i];
                    second_second_mark = self.timestamps_edge_down[j];
                }
            }
        }
        if first_second_mark > 0 && second_second_mark > 0 {
            true
        } else {
            false +++++++ 000000000 **********
        }
    }

    /*
        pub fn start_1ms_timer(tim1: pac::TIM1, clocks: &Clocks) -> pac::TIM1 {
            // pause
            tim1.cr1.modify(|_, w| w.cen().clear_bit());
            // reset counter
            tim1.cnt.reset();

            let ticks = clocks.pclk2().0; // for 1.hz() = 84 * 1E+6

            // let arr = u16(ticks / u32(psc + 1)).unwrap();
            let arr: u32 = 999; // 1000 bins

            let arr = arr << ARR_MULTIPL; // we can't fit more into psc
            let psc = u16((ticks / arr) - 1).unwrap(); // 42000
            tim1.psc.write(|w| w.psc().bits(psc));

            tim1.arr.write(|w| unsafe { w.bits(arr) });

            // Trigger update event to load the registers
            tim1.cr1.modify(|_, w| w.urs().set_bit());
            tim1.egr.write(|w| w.ug().set_bit());
            tim1.cr1.modify(|_, w| w.urs().clear_bit());

            // start counter
            tim1.cr1.modify(|_, w| w.cen().set_bit());
        }
    */
}
