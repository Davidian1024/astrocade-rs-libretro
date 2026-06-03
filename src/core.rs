use z80::Z80;
use crate::machine::IO;

use crate::machine::Machine;
use crate::machine::video;

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
    pub(crate) step_count: u64,
}

impl AstrocadeCore {
    pub fn new() -> Self {
        let io = IO {
            mem: [0u8; 65536],
            colors: [0; 8],
            horcb: 0,
            verbl: 0,
            magic: 0,
            xpand: 0,
            inmod: 0,
            infbk: 0,
            inlin: 0,
        };
        let mut machine = Machine {
            z80: Z80::<IO>::new(io),
            palette: video::build_palette(),
            frame_buffer: vec![0u32; 160 * 102],
        };
        // machine.z80.sp = 0x4FCE;
        AstrocadeCore {
            frame_count: 0,
            phase: 0.0,
            frequency: 220.0,
            waveform: Waveform::Square,
            last_button: false,
            machine,
            step_count: 0,
        }
    }
}
