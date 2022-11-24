use crate::unit::Unit;

pub struct MOS6502<U> {
    unit: U,
}

impl<U: Unit> MOS6502<U> {
    pub fn new(unit: U) -> Self {
        Self { unit }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }
}
