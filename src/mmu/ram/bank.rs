use crate::unit::Unit;

const SIZE: usize = 0x2000;

pub struct Bank([u8; SIZE]);

impl Bank {
    pub fn new() -> Self {
        Self([0; SIZE])
    }
}

impl Unit for Bank {
    fn read(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.0[address as usize] = data;
    }
}
