
use crate::{core::AstrocadeCore, types::RetroSystemInfo};

#[unsafe(no_mangle)]
pub extern "C" fn retro_api_version() -> u32 {
    eprintln!("retro_api_version(): started");
    eprintln!("retro_api_version(): finished");
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_info(info: *mut RetroSystemInfo) {
    eprintln!("retro_get_system_info(): started");
    unsafe {
        (*info).library_name = concat!(env!("CARGO_PKG_NAME"), "\0").as_ptr() as *const std::ffi::c_char;
        (*info).library_version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_BUILD_TIMESTAMP"), "\0").as_ptr() as *const std::ffi::c_char;
        (*info).valid_extensions = c"".as_ptr();
        (*info).need_fullpath = false;
        (*info).block_extract = false;
    }
    eprintln!("retro_get_system_info(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_av_info(info: *mut crate::types::RetroSystemAvInfo) {
    eprintln!("retro_get_system_av_info(): started");
    unsafe {
        (*info).geometry.base_width = 160;
        (*info).geometry.base_height = 102;
        (*info).geometry.max_width = 160;
        (*info).geometry.max_height = 102;
        (*info).geometry.aspect_ratio = 0.0;
        (*info).timing.fps = 60.0;
        (*info).timing.sample_rate = 48000.0;
    }
    eprintln!("retro_get_system_av_info(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_environment(
    cb: unsafe extern "C" fn(u32, *mut std::ffi::c_void) -> bool,
) {
    eprintln!("retro_set_environment(): started");
    unsafe {
        crate::ENVIRONMENT_CALLBACK = Some(cb);

        let mut system_dir_ptr: *const std::ffi::c_char = std::ptr::null();
        cb(
            crate::types::RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY,
            &mut system_dir_ptr as *mut *const std::ffi::c_char as *mut std::ffi::c_void,
        );
        if !system_dir_ptr.is_null() {
            let path = std::ffi::CStr::from_ptr(system_dir_ptr)
                .to_string_lossy()
                .into_owned();
            *crate::SYSTEM_DIRECTORY.lock().unwrap() = Some(path);
        }

        let mut supports_no_game = true;
        cb(
            crate::types::RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
            &mut supports_no_game as *mut bool as *mut std::ffi::c_void,
        );
    }
    eprintln!("retro_set_environment(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_video_refresh(
    cb: unsafe extern "C" fn(*const std::ffi::c_void, u32, u32, usize),
) {
    eprintln!("retro_set_video_refresh(): started");
    unsafe {
        crate::VIDEO_REFRESH_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_video_refresh(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample(cb: unsafe extern "C" fn(i16, i16)) {
    eprintln!("retro_set_audio_sample(): started");
    unsafe {
        crate::AUDIO_SAMPLE_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_audio_sample(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample_batch(
    cb: unsafe extern "C" fn(*const i16, usize) -> usize,
) {
    eprintln!("retro_set_audio_sample_batch(): started");
    unsafe {
        crate::AUDIO_SAMPLE_BATCH_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_audio_sample_batch(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_poll(cb: unsafe extern "C" fn()) {
    eprintln!("retro_set_input_poll(): started");
    unsafe {
        crate::INPUT_POLL_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_input_poll(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_state(cb: unsafe extern "C" fn(u32, u32, u32, u32) -> i16) {
    eprintln!("retro_set_input_state(): started");
    unsafe {
        crate::INPUT_STATE_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_input_state(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_init() {
    eprintln!("retro_init(): started");
    unsafe {
        if let Some(cb) = crate::ENVIRONMENT_CALLBACK {
            let mut fmt = crate::types::RETRO_PIXEL_FORMAT_XRGB8888;
            cb(
                crate::types::RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
                &mut fmt as *mut u32 as *mut std::ffi::c_void,
            );
        }
    }

    let mut core = crate::CORE.lock().unwrap();
    *core = Some(crate::core::AstrocadeCore::new());
    eprintln!("retro_init(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game(_game: *const crate::types::RetroGameInfo) -> bool {
    eprintln!("retro_load_game(): started");

    let system_dir = match crate::SYSTEM_DIRECTORY.lock().unwrap().clone() {
        Some(dir) => dir,
        None => {
            eprintln!("retro_load_game(): system directory not available");
            return false;
        }
    };

    let bios_path = format!("{}/astrocade/bioswhit.bin", system_dir);
    let bios_data = match std::fs::read(&bios_path) {
        Ok(data) => data,
        Err(e) => {
            let msg = format!("astrocade: BIOS not found.");
            eprintln!("retro_load_game(): failed to load BIOS from {}: {}", bios_path, e);
            set_message(&msg);
            return false;
        }
    };
    if bios_data.len() != 0x2000 {
        eprintln!("retro_load_game(): BIOS is wrong size (expected 8192, got {})", bios_data.len());
        return false;
    }

    {
        let mut core = crate::CORE.lock().unwrap();
        let core = core.as_mut().unwrap();
        core.machine.z80.io.mem[0x0000..0x2000].copy_from_slice(&bios_data);
    }

    eprintln!("retro_load_game(): BIOS loaded from {}", bios_path);
    eprintln!("retro_load_game(): finished");
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_run() {
    // eprintln!("retro_run(): started");

    let mut core = crate::CORE.lock().unwrap();
    let core = core.as_mut().unwrap();

    // Input

    if let Some(poll) = unsafe { crate::INPUT_POLL_CALLBACK } {
        unsafe { poll() };
    }

    // Video

    crate::machine::video::render_frame(
        &core.machine.z80.io.mem,
        &core.machine.z80.io.colors,
        core.machine.z80.io.horcb,
        core.machine.z80.io.verbl,
        &core.machine.palette,
        &mut core.machine.frame_buffer,
    );

    if let Some(cb) = unsafe { crate::VIDEO_REFRESH_CALLBACK } {
        unsafe { cb(
            core.machine.frame_buffer.as_ptr() as *const std::ffi::c_void,
            160,
            102,
            160 * 4,
        ) };
    }

    // Audio

    let frames_per_frame = 48000 / 60;
    let total_samples = frames_per_frame * 2;
    let audio_buffer: Vec<i16> = vec![0i16; total_samples ];

    if let Some(cb) = unsafe { crate::AUDIO_SAMPLE_BATCH_CALLBACK } {
        unsafe { cb(audio_buffer.as_ptr(), frames_per_frame as usize) };
    }

    // Machine

    // const CYCLES_PER_FRAME: u32 = 1;
    const CYCLES_PER_FRAME: u32 = 1_789_000 / 60;

    for i in 0..CYCLES_PER_FRAME {
        if i % 5000 == 0 {
            eprintln!("step={} PC={:#06x} I={:#04x} inmod={:#04x}", 
                core.step_count, 
                core.machine.z80.pc,
                core.machine.z80.i,
                core.machine.z80.io.inmod,
            );
        }

        // let pc = core.machine.z80.pc as usize;
        // let op = core.machine.z80.io.mem[pc];
        // let next = core.machine.z80.io.mem[pc + 1];
        // let is_interesting = matches!(op, 0xD3) // OUT (n), A
        //     || (op == 0xED && matches!(next, 0x79 | 0xB3)); // OUT (C),A or OTIR

        // if is_interesting {
            // let instr = disassemble_at(&core.machine.z80.io.mem, core.machine.z80.pc);
            // eprintln!(
            //     "step={:>10} PC={:#06x} SP={:#06x} | {:<20} | verbl={:>3} horcb={:>3} colors={:?}",
            //     core.step_count,
            //     core.machine.z80.pc,
            //     core.machine.z80.sp,
            //     instr,
            //     core.machine.z80.io.verbl,
            //     core.machine.z80.io.horcb,
            //     core.machine.z80.io.colors,
            // );
        // }
        core.step_count += 1;

        core.machine.z80.step();
    }

    core.machine.z80.pulse_irq(core.machine.z80.io.infbk);

    // let instr = disassemble_at(&core.machine.z80.io.mem, core.machine.z80.pc);
    // eprintln!(
    //     "PC={:#06x} SP={:#06x} | {:<20} | verbl={:>3} horcb={:>3} colors={:?}",
    //     core.machine.z80.pc,
    //     core.machine.z80.sp,
    //     instr,
    //     core.machine.z80.io.verbl,
    //     core.machine.z80.io.horcb,
    //     core.machine.z80.io.colors,
    // );
    // core.machine.z80.step();

    // let pc = core.machine.z80.pc as usize;
    // let op = core.machine.z80.io.mem[pc];
    // let next = core.machine.z80.io.mem[pc + 1];
    // let is_interesting = matches!(op, 0xD3) // OUT (n), A
    //     || (op == 0xED && matches!(next, 0x79 | 0xB3)); // OUT (C),A or OTIR

    // if is_interesting {
    //     let instr = disassemble_at(&core.machine.z80.io.mem, core.machine.z80.pc);
    //     eprintln!(
    //         "step={:>10} PC={:#06x} SP={:#06x} | {:<20} | verbl={:>3} horcb={:>3} colors={:?}",
    //         core.step_count,
    //         core.machine.z80.pc,
    //         core.machine.z80.sp,
    //         instr,
    //         core.machine.z80.io.verbl,
    //         core.machine.z80.io.horcb,
    //         core.machine.z80.io.colors,
    //     );
    // }
    // core.step_count += 1;
    // core.machine.z80.step();

    core.frame_count += 1;

    // eprintln!("retro_run(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_deinit() {
    eprintln!("retro_deinit(): started");
    let mut core = crate::CORE.lock().unwrap();
    *core = None;
    eprintln!("retro_deinit(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unload_game() {
    eprintln!("retro_unload_game(): started");
    eprintln!("retro_unload_game(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_region() -> u32 {
    eprintln!("retro_get_region(): started");
    eprintln!("retro_get_region(): finished");
    0 // NTSC
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize_size() -> usize {
    eprintln!("retro_serialize_size(): started");
    eprintln!("retro_serialize_size(): finished");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize(_data: *mut std::ffi::c_void, _size: usize) -> bool {
    eprintln!("retro_serialize(): started");
    eprintln!("retro_serialize(): finished");
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unserialize(_data: *const std::ffi::c_void, _size: usize) -> bool {
    eprintln!("retro_unserialize(): started");
    eprintln!("retro_unserialize(): finished");
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_data(_id: u32) -> *mut std::ffi::c_void {
    eprintln!("retro_get_memory_data(): started");
    eprintln!("retro_get_memory_data(): finished");
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_size(_id: u32) -> usize {
    eprintln!("retro_get_memory_size(): started");
    eprintln!("retro_get_memory_size(): finished");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_reset() {
    eprintln!("retro_cheat_reset(): started");
    eprintln!("retro_cheat_reset(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_set(_index: u32,_enabledd: bool, _code: *const std::ffi::c_char) {
    eprintln!("retro_cheat_set(): started");
    eprintln!("retro_cheat_set(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_controller_port_device(_port: u32, _device: u32) {
    eprintln!("retro_set_controller_port_device(): started");
    eprintln!("retro_set_controller_port_device(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_reset() {
    eprintln!("retro_reset(): started");
    eprintln!("retro_reset(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game_special(
    _game_type: u32,
    _info: *const crate::types::RetroGameInfo,
    _num_info: usize,
) -> bool {
    eprintln!("retro_load_game_special(): started");
    eprintln!("retro_load_game_special(): finished");
    false
}

fn set_message(msg: &str) {
    unsafe {
        if let Some(cb) = crate::ENVIRONMENT_CALLBACK {
            let c_msg = std::ffi::CString::new(msg).unwrap();
            let mut retro_msg = crate::types::RetroMessage {
                msg: c_msg.as_ptr(),
                frames: 600,
            };
            cb(
                crate::types::RETRO_ENVIRONMENT_SET_MESSAGE,
                &mut retro_msg as *mut crate::types::RetroMessage as *mut std::ffi::c_void,
            );
        }
    }
}

fn disassemble_at(mem: &[u8; 0x10000], pc: u16) -> String {
    let pc = pc as usize;
    let op = mem[pc];
    match op {
        0x00 => "NOP".to_string(),
        0x01 => format!("LD BC, ${:04x}", u16::from_le_bytes([mem[pc+1], mem[pc+2]])),
        0x11 => format!("LD DE, ${:04x}", u16::from_le_bytes([mem[pc+1], mem[pc+2]])),
        0x21 => format!("LD HL, ${:04x}", u16::from_le_bytes([mem[pc+1], mem[pc+2]])),
        0x31 => format!("LD SP, ${:04x}", u16::from_le_bytes([mem[pc+1], mem[pc+2]])),
        0xC3 => format!("JP ${:04x}", u16::from_le_bytes([mem[pc+1], mem[pc+2]])),
        0xCD => format!("CALL ${:04x}", u16::from_le_bytes([mem[pc+1], mem[pc+2]])),
        0xC9 => "RET".to_string(),
        0xD3 => format!("OUT (${:02x}), A", mem[pc+1]),
        0xDB => format!("IN A, (${:02x})", mem[pc+1]),
        0xED => match mem[pc+1] {
            0x43 => format!("LD (${:04x}), BC", u16::from_le_bytes([mem[pc+2], mem[pc+3]])),
            0x53 => format!("LD (${:04x}), DE", u16::from_le_bytes([mem[pc+2], mem[pc+3]])),
            0x63 => format!("LD (${:04x}), HL", u16::from_le_bytes([mem[pc+2], mem[pc+3]])),
            0x73 => format!("LD (${:04x}), SP", u16::from_le_bytes([mem[pc+2], mem[pc+3]])),
            0x79 => "OUT (C), A".to_string(),
            0xB3 => "OTIR".to_string(),
            0xB9 => "CPDR".to_string(),
            _ => format!("ED ${:02x}", mem[pc+1]),
        },
        0xF3 => "DI".to_string(),
        0xFB => "EI".to_string(),
        0xFF => "RST $38".to_string(),
        0xC0 => "RET NZ".to_string(),
        0xC8 => "RET Z".to_string(),
        0xD0 => "RET NC".to_string(),
        0xD8 => "RET C".to_string(),
        0x18 => format!("JR ${:04x}", (pc as i32 + 2 + mem[pc+1] as i8 as i32) as u16),
        0x20 => format!("JR NZ, ${:04x}", (pc as i32 + 2 + mem[pc+1] as i8 as i32) as u16),
        0x28 => format!("JR Z, ${:04x}", (pc as i32 + 2 + mem[pc+1] as i8 as i32) as u16),
        _ => format!("${:02x}", op),
    }
}