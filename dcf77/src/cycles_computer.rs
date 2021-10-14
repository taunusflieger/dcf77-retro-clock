use core::time;
use stm32f4xx_hal::time::Hertz;

pub struct CyclesComputer {
    frequency: Hertz,
}

impl CyclesComputer {
    pub fn new(frequency: Hertz) -> Self {
        CyclesComputer { frequency }
    }

    pub fn to_cycles(&self, duration: time::Duration) -> u32 {
        let s_part = (duration.as_secs() as u32) * self.frequency.0;
        let mms_part = (duration.subsec_millis()) * (self.frequency.0 / 1000);
        s_part + mms_part
    }

    pub fn from_cycles(&self, ticks: u32) -> u32 {
        let Hertz(hz) = self.frequency;
        let time = (ticks as f32) / (hz as f32);
        time as u32
    }
}
