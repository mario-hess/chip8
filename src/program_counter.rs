use crate::machine::ROM_START_ADDRESS;

pub struct ProgramCounter {
    value: u16,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { value: ROM_START_ADDRESS }
    }

    pub fn next(&mut self) {
        self.value += 2;
    }

    pub fn skip_next(&mut self) {
        self.value += 4;
    }

    pub fn set_value(&mut self, value: u16) {
        self.value = value;
    }

    pub fn get_value(&self) -> u16 {
        self.value
    }
}
