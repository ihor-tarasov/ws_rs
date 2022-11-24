mod bank;
use super::MMU;
use bank::Bank;

pub fn load(mmu: &mut MMU) {
    mmu.insert(0x0000..=0x1FFF, Bank::new());
    mmu.insert(0x4000..=0x5FFF, Bank::new());
}
