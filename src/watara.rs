use std::io::Read;
use crate::{mmu::{ram, rom, MMU}, mos6502::MOS6502};

pub struct Watara {
    cpu: MOS6502<MMU>,
}

impl Watara {
    pub fn new() -> Self {
        let mut mmu = MMU::new();
        ram::load(&mut mmu);
        Self { cpu: MOS6502::new(mmu) }
    }

    pub fn load<R: Read>(&mut self, read: R) -> Result<(), rom::Error> {
        rom::load(self.cpu.unit_mut(), read)
    }
}
