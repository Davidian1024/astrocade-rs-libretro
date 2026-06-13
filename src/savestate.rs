/// Save state serialization for the Bally Astrocade core.
///
/// The Z80 crate stores register pairs in private C2Rust union fields.  The
/// struct is `#[repr(C)]`, so the in-memory layout is deterministic and we can
/// reach the pairs with pointer arithmetic.  The layout after `io: T` is:
///
///   pc(2) sp(2) ix(2) iy(2) mem_ptr(2)          — public u16 fields
///   AF(2) BC(2) DE(2) HL(2)                      — private unions (main)
///   AF'(2) BC'(2) DE'(2) HL'(2)                  — private unions (shadow)
///   i(1) r(1) iff_delay(1) interrupt_mode(1)     — public u8 fields
///   irq_data(1) irq_pending(1) nmi_pending(1)    — public u8 fields
///   halted(1) iff1(1) iff2(1)                    — public bool fields
///
/// `IO` is a variable-size field that comes first, so we can't use a fixed
/// struct offset.  Instead we capture the register-pair values by reading
/// through a raw pointer offset calculated relative to &z80.pc.
///
/// SAFETY contract: only call get/set helpers while you hold &mut Z80.  Never
/// alias.  The repr(C) guarantee makes the layout stable across compilations
/// of the same crate version.

use std::mem::size_of;
use crate::core::AstrocadeCore;
use crate::machine::IO;
use z80::Z80;

// ---------------------------------------------------------------------------
// SaveState: a flat, POD, repr(C) snapshot of all mutable emulator state.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Z80Regs {
    pub pc: u16,
    pub sp: u16,
    pub ix: u16,
    pub iy: u16,
    pub mem_ptr: u16,
    // main register pairs
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    // shadow register pairs
    pub af_: u16,
    pub bc_: u16,
    pub de_: u16,
    pub hl_: u16,
    // misc
    pub i: u8,
    pub r: u8,
    pub iff_delay: u8,
    pub interrupt_mode: u8,
    pub irq_data: u8,
    pub irq_pending: u8,
    pub nmi_pending: u8,
    pub halted: u8,   // bool serialized as u8
    pub iff1: u8,
    pub iff2: u8,
    pub _pad: u8,
}

#[repr(C)]
pub struct SaveState {
    // ── version / magic ──────────────────────────────────────
    pub magic: [u8; 4],    // b"ACSS"
    pub version: u32,      // 1

    // ── Z80 CPU ───────────────────────────────────────────────
    pub cpu: Z80Regs,

    // ── IO: memory ────────────────────────────────────────────
    pub mem: [u8; 0x10000],

    // ── IO: video hardware registers ──────────────────────────
    pub colors: [u8; 8],
    pub horcb: u8,
    pub verbl: u8,
    pub magic_reg: u8,
    pub xpand: u8,
    pub inmod: u8,
    pub infbk: u8,
    pub inlin: u8,

    // ── IO: mid-frame color tracking ──────────────────────────
    pub colors_at_frame_start: [u8; 8],
    pub current_frame_step: u32,
    pub frame_count_io: u32,
    pub step_count_io: u64,

    // ── IO: funcgen state ─────────────────────────────────────
    pub funcgen_expand_count: u8,
    pub funcgen_shift_prev_data: u8,
    pub funcgen_rotate_count: u8,
    pub funcgen_rotate_data: [u8; 4],
    pub funcgen_expand_color: [u8; 2],
    pub funcgen_intercept: u8,
    pub _pad_fg: u8,

    // ── IO: input ─────────────────────────────────────────────
    pub input: [u8; 4],
    pub knob: [u8; 4],
    pub keypad: [u8; 4],

    // ── IO: audio chip registers ──────────────────────────────
    pub sound_reg: [u8; 8],
    pub audio_sample_acc: u32,
    pub master_count: u8,
    pub _pad_a: u8,
    pub vibrato_clock: u16,
    pub noise_clock: u8,
    pub _pad_b: u8,
    pub noise_state: u16,
    pub a_count: u8,
    pub a_state: u8,
    pub b_count: u8,
    pub b_state: u8,
    pub c_count: u8,
    pub c_state: u8,
    pub _pad_c: u16,
    pub chip_remainder: u32,

    // ── AstrocadeCore ─────────────────────────────────────────
    pub frame_count: u32,
    pub step_count: u64,
}

pub const SAVE_STATE_SIZE: usize = size_of::<SaveState>();

// ---------------------------------------------------------------------------
// Register-pair access via repr(C) pointer arithmetic.
//
// Z80<T> layout after `io: T`:
//   pc(2) sp(2) ix(2) iy(2) mem_ptr(2) af(2) bc(2) de(2) hl(2)
//   af'(2) bc'(2) de'(2) hl'(2)
//   i(1) r(1) iff_delay(1) interrupt_mode(1)
//   irq_data(1) irq_pending(1) nmi_pending(1)
//   halted(1) iff1(1) iff2(1)
//
// We read relative to the address of `z80.pc`, which is the first public
// field immediately after `io`, so offsets from &z80.pc are well-defined.
// ---------------------------------------------------------------------------

/// Read all Z80 register values into a `Z80Regs` snapshot.
///
/// # Safety
/// Caller must ensure `z80` is valid and not aliased.
pub fn capture_z80_regs(z80: &Z80<IO>) -> Z80Regs {
    // Safe public fields
    let pc = z80.pc;
    let sp = z80.sp;
    let ix = z80.ix;
    let iy = z80.iy;

    // mem_ptr is public on the z80 crate
    let mem_ptr = z80.mem_ptr;

    // The 8 private union register pairs sit right after mem_ptr in memory.
    // Each union is 2 bytes (repr(C)).  Offset from &z80.pc:
    //   pc(0) sp(2) ix(4) iy(6) mem_ptr(8) → pairs start at offset 10.
    let pc_ptr = &z80.pc as *const u16;
    let pairs = unsafe { pc_ptr.add(5) } as *const u16; // 5 u16s = 10 bytes
    let af  = unsafe { pairs.add(0).read_unaligned() };
    let bc  = unsafe { pairs.add(1).read_unaligned() };
    let de  = unsafe { pairs.add(2).read_unaligned() };
    let hl  = unsafe { pairs.add(3).read_unaligned() };
    let af_ = unsafe { pairs.add(4).read_unaligned() };
    let bc_ = unsafe { pairs.add(5).read_unaligned() };
    let de_ = unsafe { pairs.add(6).read_unaligned() };
    let hl_ = unsafe { pairs.add(7).read_unaligned() };

    Z80Regs {
        pc, sp, ix, iy, mem_ptr,
        af, bc, de, hl,
        af_, bc_, de_, hl_,
        i:                z80.i,
        r:                z80.r,
        iff_delay:        z80.iff_delay,
        interrupt_mode:   z80.interrupt_mode,
        irq_data:         z80.irq_data,
        irq_pending:      z80.irq_pending,
        nmi_pending:      z80.nmi_pending,
        halted:           z80.halted as u8,
        iff1:             z80.iff1 as u8,
        iff2:             z80.iff2 as u8,
        _pad: 0,
    }
}

/// Restore Z80 register values from a `Z80Regs` snapshot.
///
/// # Safety
/// Caller must ensure `z80` is valid and not aliased.
pub fn restore_z80_regs(z80: &mut Z80<IO>, regs: &Z80Regs) {
    z80.pc             = regs.pc;
    z80.sp             = regs.sp;
    z80.ix             = regs.ix;
    z80.iy             = regs.iy;
    z80.mem_ptr        = regs.mem_ptr;

    let pc_ptr = &mut z80.pc as *mut u16;
    let pairs = unsafe { pc_ptr.add(5) } as *mut u16;
    unsafe { pairs.add(0).write_unaligned(regs.af) };
    unsafe { pairs.add(1).write_unaligned(regs.bc) };
    unsafe { pairs.add(2).write_unaligned(regs.de) };
    unsafe { pairs.add(3).write_unaligned(regs.hl) };
    unsafe { pairs.add(4).write_unaligned(regs.af_) };
    unsafe { pairs.add(5).write_unaligned(regs.bc_) };
    unsafe { pairs.add(6).write_unaligned(regs.de_) };
    unsafe { pairs.add(7).write_unaligned(regs.hl_) };

    z80.i              = regs.i;
    z80.r              = regs.r;
    z80.iff_delay      = regs.iff_delay;
    z80.interrupt_mode = regs.interrupt_mode;
    z80.irq_data       = regs.irq_data;
    z80.irq_pending    = regs.irq_pending;
    z80.nmi_pending    = regs.nmi_pending;
    z80.halted         = regs.halted != 0;
    z80.iff1           = regs.iff1   != 0;
    z80.iff2           = regs.iff2   != 0;
}

// ---------------------------------------------------------------------------
// Public serialize / unserialize
// ---------------------------------------------------------------------------

/// Capture the full emulator state into a `SaveState`.
pub fn serialize(core: &AstrocadeCore) -> SaveState {
    let z80 = &core.machine.z80;
    let io  = &z80.io;

    let cpu = capture_z80_regs(z80);

    let ss = SaveState {
        magic: *b"ACSS",
        version: 1,
        cpu,
        mem: io.mem,
        colors: io.colors,
        horcb: io.horcb,
        verbl: io.verbl,
        magic_reg: io.magic,
        xpand: io.xpand,
        inmod: io.inmod,
        infbk: io.infbk,
        inlin: io.inlin,
        colors_at_frame_start: io.colors_at_frame_start,
        current_frame_step: io.current_frame_step,
        frame_count_io: io.frame_count,
        step_count_io: io.step_count,
        funcgen_expand_count: io.funcgen_expand_count,
        funcgen_shift_prev_data: io.funcgen_shift_prev_data,
        funcgen_rotate_count: io.funcgen_rotate_count,
        funcgen_rotate_data: io.funcgen_rotate_data,
        funcgen_expand_color: io.funcgen_expand_color,
        funcgen_intercept: io.funcgen_intercept.get(),
        _pad_fg: 0,
        input: io.input,
        knob: io.knob,
        keypad: io.keypad,
        sound_reg: io.sound_reg,
        audio_sample_acc: io.audio_sample_acc,
        master_count: io.master_count,
        _pad_a: 0,
        vibrato_clock: io.vibrato_clock,
        noise_clock: io.noise_clock,
        _pad_b: 0,
        noise_state: io.noise_state,
        a_count: io.a_count,
        a_state: io.a_state,
        b_count: io.b_count,
        b_state: io.b_state,
        c_count: io.c_count,
        c_state: io.c_state,
        _pad_c: 0,
        chip_remainder: io.chip_remainder,
        frame_count: core.frame_count,
        step_count: core.step_count,
    };

    // color_events, audio_buffer, sound_reg_shadow, bitswap, palette,
    // and frame_buffer are all transient — not saved.
    ss
}

/// Restore the full emulator state from a `SaveState`.
///
/// Returns `false` if the magic/version check fails.
pub fn unserialize(core: &mut AstrocadeCore, ss: &SaveState) -> bool {
    if &ss.magic != b"ACSS" || ss.version != 1 {
        return false;
    }

    let z80 = &mut core.machine.z80;
    restore_z80_regs(z80, &ss.cpu);

    let io = &mut z80.io;
    io.mem                      = ss.mem;
    io.colors                   = ss.colors;
    io.horcb                    = ss.horcb;
    io.verbl                    = ss.verbl;
    io.magic                    = ss.magic_reg;
    io.xpand                    = ss.xpand;
    io.inmod                    = ss.inmod;
    io.infbk                    = ss.infbk;
    io.inlin                    = ss.inlin;
    io.colors_at_frame_start    = ss.colors_at_frame_start;
    io.current_frame_step       = ss.current_frame_step;
    io.frame_count              = ss.frame_count_io;
    io.step_count               = ss.step_count_io;
    io.funcgen_expand_count     = ss.funcgen_expand_count;
    io.funcgen_shift_prev_data  = ss.funcgen_shift_prev_data;
    io.funcgen_rotate_count     = ss.funcgen_rotate_count;
    io.funcgen_rotate_data      = ss.funcgen_rotate_data;
    io.funcgen_expand_color     = ss.funcgen_expand_color;
    io.funcgen_intercept        = std::cell::Cell::new(ss.funcgen_intercept);
    io.input                    = ss.input;
    io.knob                     = ss.knob;
    io.keypad                   = ss.keypad;
    io.sound_reg                = ss.sound_reg;
    io.audio_sample_acc         = ss.audio_sample_acc;
    io.master_count             = ss.master_count;
    io.vibrato_clock            = ss.vibrato_clock;
    io.noise_clock              = ss.noise_clock;
    io.noise_state              = ss.noise_state;
    io.a_count                  = ss.a_count;
    io.a_state                  = ss.a_state;
    io.b_count                  = ss.b_count;
    io.b_state                  = ss.b_state;
    io.c_count                  = ss.c_count;
    io.c_state                  = ss.c_state;
    io.chip_remainder           = ss.chip_remainder;

    // Transient state: flush and let the next frame rebuild
    io.audio_buffer.clear();
    io.color_events.clear();

    core.frame_count = ss.frame_count;
    core.step_count  = ss.step_count;

    true
}