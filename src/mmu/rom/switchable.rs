use super::{bank::Bank, bank_id::BankID};
use crate::{
    mmu::MMU,
    unit::{Unit, UnitRef, UnitRefMut},
};

const BANKS_MAX: usize = 4;

struct Switchable {
    banks: [UnitRef<Bank>; BANKS_MAX],
    bank_id: UnitRefMut<BankID>,
}

impl Switchable {
    fn bank_id(&self) -> usize {
        self.bank_id.get().get() as usize
    }
}

impl Unit for Switchable {
    fn read(&self, address: u16) -> u8 {
        self.banks[self.bank_id()].read(address)
    }

    fn write(&mut self, _address: u16, _data: u8) {}
}

fn fill_mirrors(banks: Vec<UnitRef<Bank>>) -> [UnitRef<Bank>; BANKS_MAX] {
    match banks.len() {
        1 => [banks[0].clone(), banks[0].clone(), banks[0].clone(), banks[0].clone()],
        2 => [banks[0].clone(), banks[1].clone(), banks[0].clone(), banks[1].clone()],
        3 => [banks[0].clone(), banks[1].clone(), banks[2].clone(), banks[0].clone()],
        4 => [banks[0].clone(), banks[1].clone(), banks[2].clone(), banks[3].clone()],
        _ => panic!("Incorrect banks count"),
    }
}

pub fn assign(mmu: &mut MMU, banks: Vec<UnitRef<Bank>>) {
    let banks = fill_mirrors(banks);
    let bank_id = UnitRefMut::new(BankID::new());
    mmu.insert(0x2026..=0x2026, bank_id.clone());
    println!("[Register][BANK_ID] Allocated. Address [0x2026]");
    mmu.insert(0x8000..=0xBFFF, Switchable { banks, bank_id });
    println!("[ROM] Assigned switchable banks to [0x8000-0xBFFF]");
}
