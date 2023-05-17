pub struct Registers {
    vn: [u8; 16],
    i: u16,
    stack: Vec::<u16>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            vn: [0; 16],
            i: 0,
            stack: Vec::new(),
        }
    }

    pub fn get_vn(&self, index: u8) -> u8 {
        self.vn[index as usize]
    }

    pub fn set_vn(&mut self, index: u8, value: u8) {
        self.vn[index as usize] = value;
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn set_i(&mut self, value: u16) {
        self.i = value;
    }
    
    pub fn stack_pop(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack.push(value);
    }
}
