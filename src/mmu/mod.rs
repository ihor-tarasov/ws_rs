pub mod ram;
mod rangemap;
pub mod rom;

pub use rom::Error;
use std::ops::RangeInclusive;

use crate::unit::{Unit, UnitHolder};

#[cfg(test)]
mod rangemap_tests;

pub struct MMU {
    units: rangemap::RangeMap<u16, UnitHolder>,
}

impl MMU {
    pub fn new() -> Self {
        Self {
            units: rangemap::RangeMap::new(),
        }
    }

    pub fn insert<U: Unit + 'static>(&mut self, range: RangeInclusive<u16>, unit: U) {
        self.units
            .insert(range.clone(), UnitHolder::new(unit, *range.start()));
    }
}

impl Unit for MMU {
    fn read(&self, address: u16) -> u8 {
        if let Some(unit) = self.units.get(address) {
            unit.read(address)
        } else {
            panic!("[MMU] Not units registered at address 0x{address:4X} for read.")
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        if let Some(unit) = self.units.get_mut(address) {
            unit.write(address, data)
        } else {
            panic!("[MMU] Not units registered at address 0x{address:4X} for write.")
        }
    }
}
