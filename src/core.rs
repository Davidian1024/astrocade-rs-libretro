use z80::Z80;
use crate::machine::IO;

use crate::machine::Machine;


pub enum Waveform {
    Square,
    Pulse,
    Sawtooth,
}

pub struct AstrocadeCore {
    pub(crate) frame_count: u32,
    pub(crate) phase: f32,
    pub(crate) frequency: f32,
    pub(crate) waveform: Waveform,
    pub(crate) last_button: bool,
    pub(crate) machine: Machine,
}

impl AstrocadeCore {
    pub fn new() -> Self {
        let io = IO { mem: [0u8; 65536] };
        let machine = Machine { z80: Z80::<IO>::new(io) };
        AstrocadeCore {
            frame_count: 0,
            phase: 0.0,
            frequency: 220.0,
            waveform: Waveform::Square,
            last_button: false,
            machine,
        }
    }
}
