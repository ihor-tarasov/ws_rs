use crate::{utils::bus::BusRef, cpu::Bus, MMU};

use super::bank::Bank;

pub const SIZE: usize = 0x2000;

struct First(BusRef<Bank>);

impl Bus for First {
    fn read(&self, address: u16) -> u8 {
        debug_assert!((address as usize) < SIZE);
        self.0.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        debug_assert!((address as usize) < SIZE);
        self.0.write(address, data)
    }
}

pub fn assign(mmu: &mut MMU, bank: BusRef<Bank>) {
    mmu.insert(0x6000..=0x7FFF, bank);
    println!("[ROM][BANK0] Assigned to address [0x6000-0x7FFF]");
}
