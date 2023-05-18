use std::fs::File;
use std::io::{Error, Read};

pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn build(path: &str) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Ok(Self { data })
    }
}
