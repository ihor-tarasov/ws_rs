mod mmu;
mod cpu;
mod ram;
mod rom;
mod utils;

use std::io::Read;

use cpu::CPU;
pub use mmu::MMU;

pub struct Watara {
    cpu: CPU<MMU>,
}

impl Watara {
    pub fn new() -> Self {
        let mut mmu = MMU::new();
        ram::load(&mut mmu);
        Self { cpu: CPU::new(mmu) }
    }

    pub fn load<R: Read>(&mut self, read: R) -> Result<(), rom::Error> {
        rom::load(self.cpu.bus_mut(), read)
    }
}
