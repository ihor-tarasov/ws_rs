mod bus_holder;
use crate::{cpu::Bus, utils::RangeMap};
use std::ops::RangeInclusive;

use self::bus_holder::BusHolder;

pub struct MMU(RangeMap<u16, BusHolder>);

impl MMU {
    pub fn new() -> Self {
        Self(RangeMap::new())
    }

    pub fn insert<B: Bus + 'static>(&mut self, range: RangeInclusive<u16>, bus: B) {
        self.0
            .insert(range.clone(), BusHolder::new(bus, *range.start()));
    }
}

impl Bus for MMU {
    fn read(&self, address: u16) -> u8 {
        if let Some(unit) = self.0.get(address) {
            unit.read(address)
        } else {
            panic!("[MMU] Not units registered at address 0x{address:4X} for read.")
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        if let Some(unit) = self.0.get_mut(address) {
            unit.write(address, data)
        } else {
            panic!("[MMU] Not units registered at address 0x{address:4X} for write.")
        }
    }
}
