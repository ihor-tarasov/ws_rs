use crate::cpu::Bus;

pub struct BankID(u8);

impl BankID {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl Bus for BankID {
    fn read(&self, address: u16) -> u8 {
        debug_assert_eq!(address, 0);
        self.get()
    }

    fn write(&mut self, address: u16, data: u8) {
        debug_assert_eq!(address, 0);
        self.0 = (data & 0x60) >> 5;
    }
}
