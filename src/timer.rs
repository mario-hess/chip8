use std::time::Instant;

const TICK_DURATION_MS: u128 = 16;

pub struct Timer {
    delay_timer: u8,
    last_timer_update: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            last_timer_update: Instant::now(),
        }
    }

    pub fn get_delay_timer(&mut self) -> u8 {
        let elapsed_time = Instant::now().duration_since(self.last_timer_update).as_millis();
        let ticks = elapsed_time / TICK_DURATION_MS;

        if ticks >= self.delay_timer as u128 {
            0
        } else {
            self.delay_timer - ticks as u8
        }
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
        self.last_timer_update = Instant::now();
    }
}

