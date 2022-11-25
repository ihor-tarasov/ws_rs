pub trait Unit {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}
