pub trait Unit {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

pub struct UnitHolder {
    unit: Box<dyn Unit>,
    offset: u16,
}

impl UnitHolder {
    pub fn new<U: Unit + 'static>(unit: U, offset: u16) -> Self {
        Self {
            unit: Box::new(unit),
            offset,
        }
    }
}

impl Unit for UnitHolder {
    fn read(&self, address: u16) -> u8 {
        self.unit.read(address - self.offset)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.unit.write(address - self.offset, data)
    }
}
