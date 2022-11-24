use std::io::Read;

mod bank;
mod bank_id;
mod first;
mod switchable;
use bank::Bank;
pub use bank::Error;

use crate::unit::UnitRef;

use super::MMU;

fn end_of_read(banks: Vec<UnitRef<Bank>>) -> Result<Vec<UnitRef<Bank>>, Error> {
    if banks.is_empty() {
        Err(Error::Empty)
    } else {
        println!(
            "[ROM] {} banks readed from file. Size: {} KiB",
            banks.len(),
            banks.len() * bank::SIZE / 1024,
        );
        Ok(banks)
    }
}

fn process_error(error: Error, banks: Vec<UnitRef<Bank>>) -> Result<Vec<UnitRef<Bank>>, Error> {
    match error {
        Error::ZeroSize => end_of_read(banks),
        _ => Err(error),
    }
}

fn load_banks<R: Read>(mut read: R) -> Result<Vec<UnitRef<Bank>>, Error> {
    let mut banks = Vec::new();
    loop {
        match Bank::new(&mut read) {
            Ok(bank) => banks.push(UnitRef::new(bank)),
            Err(error) => return process_error(error, banks),
        }
    }
}

fn assign_last_bank(mmu: &mut MMU, banks: &Vec<UnitRef<Bank>>) {
    let last_index = banks.len() - 1;
    mmu.insert(0xC000..=0xFFFF, banks[last_index].clone());
    println!("[ROM][BANK{last_index}] Assigned as last bank to address [0xC000-0xFFFF]");
}

pub fn load<R: Read>(mmu: &mut MMU, read: R) -> Result<(), Error> {
    let banks = load_banks(read)?;
    first::assign(mmu, banks[0].clone());
    assign_last_bank(mmu, &banks);
    switchable::assign(mmu, banks);
    Ok(())
}
