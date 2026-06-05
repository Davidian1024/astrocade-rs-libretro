use crate::machine::IO;
use z80::Z80;

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
    pub(crate) irq_pending_cycles: u32,
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
            funcgen_expand_count: 0,
            funcgen_shift_prev_data: 0,
            funcgen_rotate_count: 0,
            funcgen_rotate_data: [0u8; 4],
            funcgen_expand_color: [0u8; 2],
            input: [0u8; 4],
            keypad: [0u8; 4],
            color_events: vec![],
            current_frame_step: 0,
            sound_reg: [0u8; 8],
            master_count: 0,
            vibrato_clock: 0,
            noise_clock: 0,
            noise_state: 0,
            a_count: 0,
            a_state: 0,
            b_count: 0,
            b_state: 0,
            c_count: 0,
            c_state: 0,
            bitswap: crate::machine::audio::build_bitswap(),
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
            irq_pending_cycles: 0,
        }
    }
}
