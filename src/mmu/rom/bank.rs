use std::io::{self, Read};

use crate::unit::Unit;

#[derive(Debug, PartialEq)]
pub enum Error {
    Interrupted,
    InvalidBankSize,
    ZeroSize,
    Empty,
    MoreThan4Banks,
}

pub const SIZE: usize = 0x4000;

fn check_size(size: usize) -> Result<(), Error> {
    if size == 0 {
        Err(Error::ZeroSize)
    } else if size != SIZE {
        Err(Error::InvalidBankSize)
    } else {
        Ok(())
    }
}

fn check_io_result(result: io::Result<usize>) -> Result<(), Error> {
    match result {
        Ok(size) => check_size(size),
        Err(_) => Err(Error::Interrupted),
    }
}

pub struct Bank([u8; SIZE]);

impl Bank {
    pub fn new<R: Read>(read: &mut R) -> Result<Self, Error> {
        let mut result = [0; SIZE];
        check_io_result(read.read(&mut result))?;
        Ok(Self(result))
    }
}

impl Unit for Bank {
    fn read(&self, address: u16) -> u8 {
        debug_assert!((address as usize) < SIZE);
        self.0[address as usize]
    }

    fn write(&mut self, address: u16, _data: u8) {
        debug_assert!((address as usize) < SIZE);
    }
}
