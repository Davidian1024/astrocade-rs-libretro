use z80::{Z80, Z80_io};

pub struct IO {
    pub mem: [u8; 0x10000],
}

impl Z80_io for IO {
    fn read_byte(&self, addr: u16) -> u8 { self.mem[addr as usize] }
    fn write_byte(&mut self, addr: u16, value: u8) { self.mem[addr as usize] = value }
}

pub struct Machine {
    pub(crate) z80: Z80<IO>,
}