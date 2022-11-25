mod bank;
use bank::Bank;
use crate::MMU;

pub fn load(mmu: &mut MMU) {
    mmu.insert(0x0000..=0x1FFF, Bank::new());
    println!("[RAM][BANK0] Allocated. Address: [0x0000-0x1FFF], Size: 8 KiB");
    mmu.insert(0x4000..=0x5FFF, Bank::new());
    println!("[RAM][BANK1] Allocated. Address: [0x4000-0x5FFF], Size: 8 KiB");
}
