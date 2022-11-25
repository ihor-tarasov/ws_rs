use super::{bank::Bank, bank_id::BankID};
use crate::{
    mmu::{
        utils::{UnitRef, UnitRefMut},
        MMU,
    },
    unit::Unit,
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

const BANK_MAP: [[usize; 4]; 4] = [[0, 0, 0, 0], [0, 1, 0, 1], [0, 1, 2, 0], [0, 1, 2, 3]];

fn fill_mirrors(banks: Vec<UnitRef<Bank>>) -> [UnitRef<Bank>; BANKS_MAX] {
    let indexes = BANK_MAP[banks.len() - 1];
    [
        banks[indexes[0]].clone(),
        banks[indexes[1]].clone(),
        banks[indexes[2]].clone(),
        banks[indexes[3]].clone(),
    ]
}

pub fn assign(mmu: &mut MMU, banks: Vec<UnitRef<Bank>>) {
    let banks = fill_mirrors(banks);
    let bank_id = UnitRefMut::new(BankID::new());
    mmu.insert(0x2026..=0x2026, bank_id.clone());
    println!("[Register][BANK_ID] Allocated. Address [0x2026]");
    mmu.insert(0x8000..=0xBFFF, Switchable { banks, bank_id });
    println!("[ROM] Assigned switchable banks to [0x8000-0xBFFF]");
}
