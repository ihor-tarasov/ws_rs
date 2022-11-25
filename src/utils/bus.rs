use std::{rc::Rc, cell::{RefCell, Ref}};

use crate::cpu::Bus;

pub struct BusRef<B>(Rc<B>);

impl<B> BusRef<B> {
    pub fn new(bus: B) -> Self {
        Self(Rc::new(bus))
    }
}

impl<B: Bus> Bus for BusRef<B> {
    fn read(&self, address: u16) -> u8 {
        self.0.read(address)
    }

    fn write(&mut self, _address: u16, _data: u8) {}
}

impl<B: Bus> Clone for BusRef<B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct BusRefMut<B>(Rc<RefCell<B>>);

impl<B> BusRefMut<B> {
    pub fn new(bus: B) -> Self {
        Self(Rc::new(RefCell::new(bus)))
    }

    pub fn get(&self) -> Ref<B> {
        self.0.borrow()
    }
}

impl<B: Bus> Bus for BusRefMut<B> {
    fn read(&self, address: u16) -> u8 {
        self.0.borrow().read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.0.borrow_mut().write(address, data)
    }
}

impl<B> Clone for BusRefMut<B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
