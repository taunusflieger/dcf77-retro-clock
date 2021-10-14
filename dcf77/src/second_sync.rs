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
//use ringbuffer::RingBuffer;
//use rtt_target::rprintln;

pub enum Edge {
    Falling,
    Rising,
}

pub struct SecondSync {
    timestamps_edge_down: [u32; 300],
    timestamps_edge_up: [u32; 300],
    edge_down_idx: usize,
    edge_up_idx: usize,
    cycles_computer: CyclesComputer,
}

impl SecondSync {
    pub fn new(cycles_computer: CyclesComputer) -> Self {
        SecondSync {
            timestamps_edge_down: [0; 300],
            timestamps_edge_up: [0; 300],
            edge_down_idx: 0,
            edge_up_idx: 0,
            cycles_computer,
        }
    }

    pub fn register_transition(&mut self, signal: Edge, now: u32) {
        match signal {
            Edge::Falling => {
                self.timestamps_edge_down[self.edge_down_idx] =
                    self.cycles_computer.from_cycles(now);
            }
            Edge::Rising => {
                self.timestamps_edge_up[self.edge_up_idx] = self.cycles_computer.from_cycles(now);
            }
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
