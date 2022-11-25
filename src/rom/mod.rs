use std::io::Read;

mod bank;
mod bank_id;
mod first;
mod switchable;
use bank::Bank;
pub use bank::Error;

use crate::utils::bus::BusRef;

use super::MMU;

type Banks = Vec<BusRef<Bank>>;

fn end_of_read(banks: Banks) -> Result<Banks, Error> {
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

fn process_error(error: Error, banks: Banks) -> Result<Banks, Error> {
    match error {
        Error::ZeroSize => end_of_read(banks),
        _ => Err(error),
    }
}

fn load_banks<R: Read>(mut read: R) -> Result<Banks, Error> {
    let mut banks = Vec::new();
    loop {
        match Bank::new(&mut read) {
            Ok(bank) => {
                if banks.len() > 4 {
                    return Err(Error::MoreThan4Banks);
                } else {
                    banks.push(BusRef::new(bank))
                }
            }
            Err(error) => return process_error(error, banks),
        }
    }
}

fn assign_last_bank(mmu: &mut MMU, banks: &Banks) {
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
