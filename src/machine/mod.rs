use z80::{Z80, Z80_io};

pub mod video;

pub struct IO {
    pub mem: [u8; 0x10000],
    // Color registers: [COL0R, COL1R, COL2R, COL3R, COL0L, COL1L, COL2L, COL3L]
    // ports $00-$07
    pub colors: [u8; 8],
    // Horizontal color boundary, port $09
    pub horcb: u8,
    // Vertical blank line, port $0A
    pub verbl: u8,
    // Magic register, port $0C
    pub magic: u8,
    // Expander register, port $19
    pub xpand: u8,
}

impl Z80_io for IO {
    fn read_byte(&self, addr: u16) -> u8 { self.mem[addr as usize] }

    fn write_byte(&mut self, addr: u16, value: u8) { self.mem[addr as usize] = value }

    fn port_out(&mut self, addr: u16, value: u8) {
        match addr as u8 {
            0x00..=0x07 => self.colors[addr as usize] = value,
            0x09 => self.horcb = value,
            0x0A => self.verbl = value,
            0x0C => self.magic = value,
            0x19 => self.xpand = value,
            _ => {}
        }
    }

    fn port_in(&self, _addr: u16) -> u8 {
        // controllers and keyboard — stub for now
        0xFF
    }
}

pub struct Machine {
    pub(crate) z80: Z80<IO>,
    pub(crate) palette: Box<[u32; 512]>,
    pub(crate) frame_buffer: Vec<u32>,
}