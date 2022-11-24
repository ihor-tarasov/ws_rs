use std::io::Read;

mod bank;
mod bank_id;
use bank::Bank;
pub use bank::Error;
use bank_id::BankID;

use crate::unit::Unit;

use super::MMU;

struct ROM {
    banks: Box<[Bank]>,
    bank_id: BankID,
}

impl ROM {
    fn new(banks: Box<[Bank]>) -> Self {
        Self {
            banks,
            bank_id: BankID::new(),
        }
    }
}

fn load_banks<R: Read>(mut read: R) -> Result<Box<[Bank]>, Error> {
    let mut banks = Vec::new();
    loop {
        match Bank::new(&mut read) {
            Ok(bank) => banks.push(bank),
            Err(error) => {
                return match error {
                    Error::ZeroSize => {
                        if banks.is_empty() {
                            Err(Error::Empty)
                        } else {
                            Ok(banks.into_boxed_slice())
                        }
                    }
                    _ => Err(error),
                }
            }
        }
    }
}

pub fn load<R: Read>(mmu: &mut MMU, read: R) -> Result<(), Error> {
    let banks = load_banks(read)?;

    mmu.insert(0x6000..=0x7FFF, banks[0].clone());

    if banks.len() > 1 {
        mmu.insert(0xC000..=0xDFFF, banks[banks.len() - 2].clone());
        mmu.insert(0xE000..=0xFFFF, banks[banks.len() - 1].clone());
    }

    let rom = ROM::new(banks);

    mmu.insert(0x2026..=0x2026, rom.bank_id.clone());
    mmu.insert(0x8000..=0xBFFF, rom);

    Ok(())
}

impl Unit for ROM {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.banks[self.bank_id.get() as usize * 2 + 0].read(address),
            0x2000..=0x3FFF => {
                self.banks[self.bank_id.get() as usize * 2 + 1].read(address - 0x2000)
            }
            _ => panic!("[ROM] Unsupported read address: 0x{:4X}", address + 0x8000),
        }
    }

    fn write(&mut self, _address: u16, _data: u8) {}
}
