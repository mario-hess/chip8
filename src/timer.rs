
use std::time::Instant;

pub struct Timer {
    delay_timer: u8,
    elapsed_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            elapsed_time: Instant::now(),
        }
    }

    pub fn get_delay_timer(&mut self) -> u8 {
        let ms_diff = (Instant::now() - self.elapsed_time).as_millis();
        let ticks = ms_diff / 16;

        if ticks >= self.delay_timer as u128 {
            0
        } else {
            self.delay_timer - ticks as u8
        }
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
        self.elapsed_time = Instant::now();
    }
}