use z80::{Z80, Z80_io};
use std::cell::Cell;

use crate::{CYCLES_PER_FRAME, SAMPLES_PER_FRAME};

pub mod audio;
pub mod video;

pub struct IO {
    pub mem: [u8; 0x10000],
    
    pub colors: [u8; 8],  // Ports $00-$07: Color registers: [COL0R, COL1R, COL2R, COL3R, COL0L, COL1L, COL2L, COL3L]
    pub horcb: u8,        // Port      $09: Horizontal color boundary
    pub verbl: u8,        // Port      $0A: Vertical blank line
    pub magic: u8,        // Port      $0C: Magic register
    pub xpand: u8,        // Port      $19: Expander register
    pub infbk: u8,        // Port $0D: Interrupt feedback / vector byte
    pub inmod: u8,        // Port $0E: Interrupt enable and mode (bit 3 = scanline int enable)
    pub inlin: u8,        // Port $0F: Interrupt scanline number

    pub colors_at_frame_start: [u8; 8],
    pub color_events: Vec<(u32, usize, u8)>, // (frame_step, register_index, value)
    pub current_frame_step: u32,
    pub frame_count: u32,
    pub step_count: u64,

    // Magic memory function generator state
    pub funcgen_expand_count: u8,      // Flip-flop for expand mode
    pub funcgen_shift_prev_data: u8,   // Previous byte for shift spillover
    pub funcgen_rotate_count: u8,      // Counter for rotate mode
    pub funcgen_rotate_data: [u8; 4],  // Accumulated data for rotate
    pub funcgen_expand_color: [u8; 2], // Colors from xpand register
    pub funcgen_intercept: Cell<u8>,

    pub input: [u8; 4], // Handle state for players 1-4
    pub knob: [u8; 4],
    pub keypad: [u8; 4],

    // Sound chip registers
    pub sound_reg: [u8; 8],
    pub sound_reg_shadow: [u8; 8],
    pub audio_buffer: Vec<i16>,        // grows to frames_per_frame each frame
    pub audio_sample_acc: u32,         // fractional sample accumulator (fixed-point)

    // Oscillator state
    pub master_count: u8,
    pub vibrato_clock: u16,
    pub noise_clock: u8,
    pub noise_state: u16,
    pub a_count: u8,
    pub a_state: u8,
    pub b_count: u8,
    pub b_state: u8,
    pub c_count: u8,
    pub c_state: u8,

    // Bitswap table for noise
    pub bitswap: [u8; 256],
    pub chip_remainder: u32,
}

impl Z80_io for IO {
    fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        let addr_usize = addr as usize;
        if addr_usize < 0x4000 {
            self.funcgen_write(addr_usize, value);
        } else {
            self.mem[addr_usize] = value;
        }
    }

    fn port_out(&mut self, addr: u16, value: u8) {
        match addr as u8 {
            0x00..=0x07 => {
                let reg = (addr as u8) as usize;
                self.colors[reg] = value;
                self.color_events
                    .push((self.current_frame_step, reg, value));
            }
            0x09 => self.horcb = value,
            0x0A => self.verbl = value,
            0x0B => {
                let reg = ((addr >> 8) as usize + 7) & 0x07;
                self.colors[reg] = value;
                self.color_events
                    .push((self.current_frame_step, reg, value));
            }
            0x0C => {
                self.magic = value;
                self.funcgen_expand_count = 0;
                self.funcgen_rotate_count = 0;
                self.funcgen_shift_prev_data = 0;
            }
            0x0D => self.infbk = value,   // interrupt vector byte
            0x0E => self.inmod = value,   // interrupt enable/mode
            0x0F => self.inlin = value,   // interrupt scanline
            0x10..=0x17 => {
                let reg = ((addr as u8) - 0x10) as usize;
                self.flush_audio();
                self.sound_reg[reg] = value;
            }
            0x18 => {
                let reg = ((addr >> 8) as usize + 7) & 0x07;
                self.flush_audio();
                self.sound_reg[reg] = value;
            }
            0x19 => {
                    self.xpand = value;
                    self.funcgen_expand_color[0] = value & 0x03;
                    self.funcgen_expand_color[1] = (value >> 2) & 0x03;
            }
            _ => {}
        }
    }

    fn port_in(&self, addr: u16) -> u8 {
        let port = addr as u8;
        let result = match port {
            0x00..=0x07 => 0x00, // write-only video registers
            0x08 => {
                let val = self.funcgen_intercept.get();
                self.funcgen_intercept.set(val & 0x0f); // clear latched bits[7:4] on read
                val
            }
            0x09..=0x0B => 0x00, // write-only video registers
            0x0C => 0x00,        // write-only MAGIC
            0x0D => 0x00,        // write-only INFBK (vector)
            0x0E => 0x00,        // write-only INMOD (enable/mode)
            0x0F => 0x00,        // write-only INLIN (scanline)
            0x10..=0x17 => {
                let slot = (addr & 0x07) as u8;  // use low bits, not high byte
                if slot & 0x04 != 0 {
                    // keypad
                    let bank = (slot & 0x03) as usize;
                    self.keypad[bank]
                } else {
                    // handle
                    let ctrl = (slot & 0x03) as usize;
                    if ctrl < 4 { self.input[ctrl] } else { 0x00 }
                }
            }
            0x18 => 0x00,  // sound chip read stub
            0x19..=0x1B => 0x00, // sound chip pot reads etc, stub
            0x1C..=0x1F => {
                let ctrl = (addr & 0x03) as usize;
                self.knob[ctrl]
            }
            _ => 0xFF,
        };
        result
    }
}

impl IO {
    fn funcgen_write(&mut self, offset: usize, mut data: u8) {
        let ctrl = self.magic;

        // Expand (bit 3)
        if ctrl & 0x08 != 0 {
            self.funcgen_expand_count ^= 1;
            data >>= 4 * self.funcgen_expand_count;
            data = (self.funcgen_expand_color[((data >> 3) & 1) as usize] << 6)
                | (self.funcgen_expand_color[((data >> 2) & 1) as usize] << 4)
                | (self.funcgen_expand_color[((data >> 1) & 1) as usize] << 2)
                | (self.funcgen_expand_color[((data >> 0) & 1) as usize] << 0);
        }

        let prev_data = self.funcgen_shift_prev_data;
        self.funcgen_shift_prev_data = data;

        // Rotate (bit 2) or Shift (bits 0-1)
        if ctrl & 0x04 != 0 {
            // Rotate — accumulate first 4 writes, output next 4
            if self.funcgen_rotate_count & 4 == 0 {
                self.funcgen_rotate_data[(self.funcgen_rotate_count & 3) as usize] = data;
                self.funcgen_rotate_count += 1;
                return; // don't write yet
            } else {
                let shift = 2 * ((!self.funcgen_rotate_count) & 3);
                data = (((self.funcgen_rotate_data[3] >> shift) & 3) << 6)
                    | (((self.funcgen_rotate_data[2] >> shift) & 3) << 4)
                    | (((self.funcgen_rotate_data[1] >> shift) & 3) << 2)
                    | (((self.funcgen_rotate_data[0] >> shift) & 3) << 0);
                self.funcgen_rotate_count = self.funcgen_rotate_count.wrapping_add(1);
            }
        } else {
            // Shift
            let shift = 2 * (ctrl & 0x03);
            if shift == 0 {
                // no shift, data unchanged
            } else {
                data = (data >> shift) | (prev_data << (8 - shift));
            }
        }

        // Flop (bit 6) — reverse pixel order
        if ctrl & 0x40 != 0 {
            data = (data >> 6) | ((data >> 2) & 0x0c) | ((data << 2) & 0x30) | (data << 6);
        }

        // OR / XOR (bits 4/5)
        let fb_addr = 0x4000 + offset;
        if ctrl & 0x30 != 0 {
            let old_data = self.mem[fb_addr];
            let incoming = data;
            if ctrl & 0x10 != 0 {
                data |= old_data; // OR
            } else if ctrl & 0x20 != 0 {
                data ^= old_data; // XOR
            }
            // Intercept: set a bit for each 2-bit pixel where both incoming and existing are non-zero
            let mut intercept = 0u8;
            for pix in 0..4u8 {
                let shift = pix * 2;
                if ((incoming >> shift) & 0x03) != 0 && ((old_data >> shift) & 0x03) != 0 {
                    intercept |= 1 << pix;
                }
            }
            // bits[3:0] = last write; bits[7:4] = latched (OR in, cleared on read)
            let current = self.funcgen_intercept.get();
            self.funcgen_intercept.set((current & 0xf0) | intercept | (intercept << 4));
        }

        // Write result to framebuffer
        self.mem[fb_addr] = data;
    }

    pub fn flush_audio(&mut self) {
        let target = (self.current_frame_step as u64 * SAMPLES_PER_FRAME as u64
            / CYCLES_PER_FRAME as u64) as usize;
        let current = self.audio_buffer.len();
        if target <= current {
            return;
        }
        let count = target - current;
        let mut new_samples = vec![0i16; count];
        crate::machine::audio::generate_audio(
            &self.sound_reg,
            &mut self.master_count,
            &mut self.vibrato_clock,
            &mut self.noise_clock,
            &mut self.noise_state,
            &mut self.a_count,
            &mut self.a_state,
            &mut self.b_count,
            &mut self.b_state,
            &mut self.c_count,
            &mut self.c_state,
            &self.bitswap,
            &mut self.chip_remainder,
            &mut new_samples,
        );
        self.audio_buffer.extend_from_slice(&new_samples);
    }
}

pub struct Machine {
    pub(crate) z80: Z80<IO>,
    pub(crate) palette: Box<[u32; 512]>,
    pub(crate) frame_buffer: Vec<u32>,
}