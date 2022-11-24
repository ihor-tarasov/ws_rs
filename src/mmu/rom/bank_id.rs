use std::{cell::RefCell, rc::Rc};

use crate::unit::Unit;

#[derive(Clone)]
pub struct BankID(Rc<RefCell<u8>>);

impl BankID {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(0)))
    }

    pub fn get(&self) -> u8 {
        *self.0.borrow()
    }

    fn set(&self, data: u8) {
        *self.0.borrow_mut() = data;
    } 
}

impl Unit for BankID {
    fn read(&self, _address: u16) -> u8 {
        self.get()
    }

    fn write(&mut self, _address: u16, data: u8) {
        self.set((data & 0x60) >> 5);
    }
}
