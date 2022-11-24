use std::{io::Read, rc::Rc};

use crate::unit::Unit;

#[derive(Debug, PartialEq)]
pub enum Error {
    Interrupted,
    InvalidBankSize,
    ZeroSize,
    Empty,
}

pub const SIZE: usize = 0x2000;

#[derive(Clone)]
pub struct Bank(Rc<[u8; SIZE]>);

impl Bank {
    pub fn new<R: Read>(read: &mut R) -> Result<Self, Error> {
        let mut result = [0; SIZE];

        match read.read(&mut result) {
            Ok(size) => {
                if size == 0 {
                    Err(Error::ZeroSize)
                } else if size != SIZE {
                    Err(Error::InvalidBankSize)
                } else {
                    Ok(Self(Rc::new(result)))
                }
            }
            Err(_) => Err(Error::Interrupted),
        }
    }
}

impl Unit for Bank {
    fn read(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, _address: u16, _data: u8) {}
}
