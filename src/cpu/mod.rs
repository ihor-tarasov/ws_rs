mod bus;
pub use bus::Bus;

pub struct CPU<B> {
    bus: B,
}

impl<B: Bus> CPU<B> {
    pub fn new(bus: B) -> Self {
        Self { bus }
    }

    pub fn bus(&self) -> &B {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut B {
        &mut self.bus
    }
}
