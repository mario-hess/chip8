use std::fs::File;
use std::io::Read;

pub struct Rom {
    pub data: Vec<u8>
}

impl Rom {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        Self { data }
    }
}
