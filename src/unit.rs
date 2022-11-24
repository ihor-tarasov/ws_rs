use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

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

pub struct UnitRef<U>(Rc<U>);

impl<U> UnitRef<U> {
    pub fn new(unit: U) -> Self {
        Self(Rc::new(unit))
    }
}

impl<U: Unit> Unit for UnitRef<U> {
    fn read(&self, address: u16) -> u8 {
        self.0.read(address)
    }

    fn write(&mut self, _address: u16, _data: u8) {}
}

impl<U: Unit> Clone for UnitRef<U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct UnitRefMut<U>(Rc<RefCell<U>>);

impl<U> UnitRefMut<U> {
    pub fn new(unit: U) -> Self {
        Self(Rc::new(RefCell::new(unit)))
    }

    pub fn get(&self) -> Ref<U> {
        self.0.borrow()
    }
}

impl<U: Unit> Unit for UnitRefMut<U> {
    fn read(&self, address: u16) -> u8 {
        self.0.borrow().read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.0.borrow_mut().write(address, data)
    }
}

impl<U> Clone for UnitRefMut<U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
