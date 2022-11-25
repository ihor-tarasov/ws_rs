use crate::cpu::Bus;

pub struct BusHolder {
    bus: Box<dyn Bus>,
    offset: u16,
}

impl BusHolder {
    pub fn new<U: Bus + 'static>(bus: U, offset: u16) -> Self {
        Self {
            bus: Box::new(bus),
            offset,
        }
    }
}

impl Bus for BusHolder {
    fn read(&self, address: u16) -> u8 {
        self.bus.read(address - self.offset)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.bus.write(address - self.offset, data)
    }
}
