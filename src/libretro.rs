use crate::{CYCLES_PER_FRAME, retro_log, types::{RETRO_DEVICE_ANALOG, RETRO_DEVICE_ID_ANALOG_X, RETRO_DEVICE_ID_ANALOG_Y, RETRO_DEVICE_ID_JOYPAD_R2, RETRO_DEVICE_ID_KEYBOARD_0, RETRO_DEVICE_ID_KEYBOARD_1, RETRO_DEVICE_ID_KEYBOARD_2, RETRO_DEVICE_ID_KEYBOARD_3, RETRO_DEVICE_ID_KEYBOARD_4, RETRO_DEVICE_ID_KEYBOARD_5, RETRO_DEVICE_ID_KEYBOARD_6, RETRO_DEVICE_ID_KEYBOARD_7, RETRO_DEVICE_ID_KEYBOARD_8, RETRO_DEVICE_ID_KEYBOARD_9, RETRO_DEVICE_ID_KEYBOARD_ASTERISK, RETRO_DEVICE_ID_KEYBOARD_COLON, RETRO_DEVICE_ID_KEYBOARD_COMMA, RETRO_DEVICE_ID_KEYBOARD_MINUS, RETRO_DEVICE_ID_KEYBOARD_PERIOD, RETRO_DEVICE_ID_KEYBOARD_PLUS, RETRO_DEVICE_ID_KEYBOARD_RETURN, RETRO_DEVICE_ID_KEYBOARD_SEMICOLON, RETRO_DEVICE_ID_KEYBOARD_SLASH, RETRO_DEVICE_INDEX_ANALOG_LEFT, RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_KEYBOARD, RETRO_ENVIRONMENT_GET_LOG_INTERFACE, RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY, RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, RETRO_ENVIRONMENT_SET_KEYBOARD_REPORTING, RETRO_ENVIRONMENT_SET_MESSAGE, RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, RETRO_PIXEL_FORMAT_XRGB8888, RetroGameInfo, RetroInputDescriptor, RetroLogCallback, RetroLogLevel, RetroMessage, RetroSystemAvInfo, RetroSystemInfo}};

#[cfg(feature = "debug_logging")]
use crate::debug_print;

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
        (*info).library_name =
            concat!(env!("CARGO_PKG_NAME"), "\0").as_ptr() as *const std::ffi::c_char;
        (*info).library_version = concat!(
            env!("CARGO_PKG_VERSION"),
            "-",
            env!("VERGEN_BUILD_TIMESTAMP"),
            "\0"
        )
        .as_ptr() as *const std::ffi::c_char;
        (*info).valid_extensions = c"bin".as_ptr();
        (*info).need_fullpath = false;
        (*info).block_extract = false;
    }
    eprintln!("retro_get_system_info(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_av_info(info: *mut RetroSystemAvInfo) {
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

    let mut log_cb = RetroLogCallback { log: None };
    unsafe {
        cb(
            RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
            &mut log_cb as *mut RetroLogCallback as *mut std::ffi::c_void,
        )
    };
    if log_cb.log.is_some() {
        *crate::LOG_CALLBACK.lock().unwrap() = log_cb.log;
    }

    let mut keyboard_reporting = true;
    unsafe {
        cb(
            RETRO_ENVIRONMENT_SET_KEYBOARD_REPORTING,
            &mut keyboard_reporting as *mut bool as *mut std::ffi::c_void,
        )
    };

    unsafe {
        crate::ENVIRONMENT_CALLBACK = Some(cb);

        let mut system_dir_ptr: *const std::ffi::c_char = std::ptr::null();
        cb(
            RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY,
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
            RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
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
            let mut fmt = RETRO_PIXEL_FORMAT_XRGB8888;
            cb(
                RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
                &mut fmt as *mut u32 as *mut std::ffi::c_void,
            );
        }
    }

    let mut core = crate::CORE.lock().unwrap();
    *core = Some(crate::core::AstrocadeCore::new());
    eprintln!("retro_init(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game(game: *const RetroGameInfo) -> bool {
    eprintln!("retro_load_game(): started");

    let system_dir = match crate::SYSTEM_DIRECTORY.lock().unwrap().clone() {
        Some(dir) => dir,
        None => {
            eprintln!("retro_load_game(): system directory not available");
            return false;
        }
    };

    if let Some(env) = unsafe { crate::ENVIRONMENT_CALLBACK } {
        let desc = [
            // Port 0
            RetroInputDescriptor { port: 0, device: 1, index: 0, id: 4,  description: c"Up".as_ptr() },
            RetroInputDescriptor { port: 0, device: 1, index: 0, id: 5,  description: c"Down".as_ptr() },
            RetroInputDescriptor { port: 0, device: 1, index: 0, id: 6,  description: c"Left".as_ptr() },
            RetroInputDescriptor { port: 0, device: 1, index: 0, id: 7,  description: c"Right".as_ptr() },
            RetroInputDescriptor { port: 0, device: 1, index: 0, id: 13, description: c"Trigger".as_ptr() },
            RetroInputDescriptor { port: 0, device: 5, index: 1, id: 1,  description: c"Knob".as_ptr() },
            // Port 1
            RetroInputDescriptor { port: 1, device: 1, index: 0, id: 4,  description: c"Up".as_ptr() },
            RetroInputDescriptor { port: 1, device: 1, index: 0, id: 5,  description: c"Down".as_ptr() },
            RetroInputDescriptor { port: 1, device: 1, index: 0, id: 6,  description: c"Left".as_ptr() },
            RetroInputDescriptor { port: 1, device: 1, index: 0, id: 7,  description: c"Right".as_ptr() },
            RetroInputDescriptor { port: 1, device: 1, index: 0, id: 13, description: c"Trigger".as_ptr() },
            RetroInputDescriptor { port: 1, device: 5, index: 1, id: 1,  description: c"Knob".as_ptr() },
            // Ports 2
            RetroInputDescriptor { port: 2, device: 1, index: 0, id: 4,  description: c"Up".as_ptr() },
            RetroInputDescriptor { port: 2, device: 1, index: 0, id: 5,  description: c"Down".as_ptr() },
            RetroInputDescriptor { port: 2, device: 1, index: 0, id: 6,  description: c"Left".as_ptr() },
            RetroInputDescriptor { port: 2, device: 1, index: 0, id: 7,  description: c"Right".as_ptr() },
            RetroInputDescriptor { port: 2, device: 1, index: 0, id: 13, description: c"Trigger".as_ptr() },
            RetroInputDescriptor { port: 2, device: 5, index: 1, id: 1,  description: c"Knob".as_ptr() },
            // Ports 3
            RetroInputDescriptor { port: 3, device: 1, index: 0, id: 4,  description: c"Up".as_ptr() },
            RetroInputDescriptor { port: 3, device: 1, index: 0, id: 5,  description: c"Down".as_ptr() },
            RetroInputDescriptor { port: 3, device: 1, index: 0, id: 6,  description: c"Left".as_ptr() },
            RetroInputDescriptor { port: 3, device: 1, index: 0, id: 7,  description: c"Right".as_ptr() },
            RetroInputDescriptor { port: 3, device: 1, index: 0, id: 13, description: c"Trigger".as_ptr() },
            RetroInputDescriptor { port: 3, device: 5, index: 1, id: 1,  description: c"Knob".as_ptr() },
            // Null terminator
            RetroInputDescriptor { port: 0, device: 0, index: 0, id: 0,  description: std::ptr::null() },
        ];
        unsafe {
            env(RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, desc.as_ptr() as *mut std::ffi::c_void);
        }
    }

    // Load BIOS
    let bios_path = format!("{}/astrocade/bioswhit.bin", system_dir);
    let bios_data = match std::fs::read(&bios_path) {
        Ok(data) => data,
        Err(e) => {
            let msg = format!("astrocade: BIOS not found.");
            retro_log!(RetroLogLevel::Error,
                "Failed to load BIOS from {}: {}", bios_path, e);
            set_message(&msg);
            return false;
        }
    };
    if bios_data.len() != 0x2000 {
        retro_log!(RetroLogLevel::Error,
            "BIOS wrong size (expected 8192, got {})", bios_data.len());
        return false;
    }

    {
        let mut core = crate::CORE.lock().unwrap();
        let core = core.as_mut().unwrap();

        // Load BIOS into $0000-$1FFF
        core.machine.z80.io.mem[0x0000..0x2000].copy_from_slice(&bios_data);

        // Load cart if present
        if !game.is_null() {
            unsafe {
                let size = (*game).size;
                let data = (*game).data;
                if size > 0 && !data.is_null() {
                    let cart_size = size.min(0x2000);
                    let cart_slice = std::slice::from_raw_parts(
                        data as *const u8, cart_size);
                    core.machine.z80.io.mem[0x2000..0x2000 + cart_size]
                        .copy_from_slice(cart_slice);
                    retro_log!(RetroLogLevel::Info,
                        "Cart loaded: {} bytes", cart_size);
                }
            }
        }
    }

    retro_log!(RetroLogLevel::Info, "BIOS loaded from {}", bios_path);
    eprintln!("retro_load_game(): finished");
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_run() {
    let mut core = crate::CORE.lock().unwrap();
    let core = core.as_mut().unwrap();

    // Input

    #[cfg(feature = "debug_logging")]
    let mut catch_trigger = false;
    #[cfg(feature = "debug_logging")]
    let mut catch_enter = false;

    if let Some(poll) = unsafe { crate::INPUT_POLL_CALLBACK } {
        unsafe { poll() };
    }
    if let Some(state) = unsafe { crate::INPUT_STATE_CALLBACK } {
        for port in 0..4u32 {
            let trigger = unsafe { state(port, 1, 0, RETRO_DEVICE_ID_JOYPAD_R2) != 0 };

            #[cfg(feature = "debug_logging")]
            if trigger { catch_trigger = true; }

            let left_stick_x: i16 = unsafe { state(port, RETRO_DEVICE_ANALOG, RETRO_DEVICE_INDEX_ANALOG_LEFT, RETRO_DEVICE_ID_ANALOG_X) };
            let left_stick_y: i16 = unsafe { state(port, RETRO_DEVICE_ANALOG, RETRO_DEVICE_INDEX_ANALOG_LEFT, RETRO_DEVICE_ID_ANALOG_Y) };
            let _right_stick_x: i16 = unsafe { state(port, RETRO_DEVICE_ANALOG, RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_ID_ANALOG_X) };
            let right_stick_y: i16 = unsafe { state(port, RETRO_DEVICE_ANALOG, RETRO_DEVICE_INDEX_ANALOG_RIGHT, RETRO_DEVICE_ID_ANALOG_Y) };

            let up = (left_stick_y < -10000) || unsafe { state(port, 1, 0, 4) != 0 };
            let down = (left_stick_y > 10000) || unsafe { state(port, 1, 0, 5) != 0 };
            let left = (left_stick_x < -10000) || unsafe { state(port, 1, 0, 6) != 0 };
            let right = (left_stick_x > 10000) || unsafe { state(port, 1, 0, 7) != 0 };

            core.machine.z80.io.input[port as usize] = (if up { 0x01 } else { 0x00 })
                | (if down { 0x02 } else { 0x00 })
                | (if left { 0x04 } else { 0x00 })
                | (if right { 0x08 } else { 0x00 })
                | (if trigger { 0x10 } else { 0x00 });

            let knob_value = 255 - (((right_stick_y as i32) + 32768) / 256) as u8 ;
            core.machine.z80.io.knob[port as usize] = knob_value;
        }

        let key_asterisk = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_ASTERISK, ) } != 0;
        let key_plus = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_PLUS, ) } != 0;
        let key_comma = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_COMMA, ) } != 0;
        let key_minus = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_MINUS, ) } != 0;
        let key_period = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_PERIOD, ) } != 0;
        let key_slash = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_SLASH, ) } != 0;
        let key_0 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_0, ) } != 0;
        let key_1 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_1, ) } != 0;
        let key_2 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_2, ) } != 0;
        let key_3 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_3, ) } != 0;
        let key_4 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_4, ) } != 0;
        let key_5 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_5, ) } != 0;
        let key_6 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_6, ) } != 0;
        let key_7 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_7, ) } != 0;
        let key_8 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_8, ) } != 0;
        let key_9 = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_9, ) } != 0;
        let key_colon = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_COLON, ) } != 0;
        let key_semicolon = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_SEMICOLON, ) } != 0;
        let key_enter = unsafe { state( 0, RETRO_DEVICE_KEYBOARD, 0, RETRO_DEVICE_ID_KEYBOARD_RETURN, ) } != 0;

        #[cfg(feature = "debug_logging")]
        if key_enter { catch_enter = true; }

        core.machine.z80.io.keypad[0] = (if key_enter  { 0x20 } else { 0x00 }) | (if key_plus { 0x10 } else { 0x00 }) | (if key_minus { 0x08 } else { 0x00 }) | (if key_asterisk { 0x04 } else { 0x00 }) | (if key_slash { 0x02 } else { 0x00 });
        core.machine.z80.io.keypad[1] = (if key_period { 0x20 } else { 0x00 }) | (if key_3    { 0x10 } else { 0x00 }) | (if key_6     { 0x08 } else { 0x00 }) | (if key_9        { 0x04 } else { 0x00 });
        core.machine.z80.io.keypad[2] = (if key_0      { 0x20 } else { 0x00 }) | (if key_2    { 0x10 } else { 0x00 }) | (if key_5     { 0x08 } else { 0x00 }) | (if key_8        { 0x04 } else { 0x00 });
        core.machine.z80.io.keypad[3] = (if key_1 { 0x10 } else { 0x00 }) | (if key_4 { 0x08 } else { 0x00 }) | (if key_7 { 0x04 } else { 0x00 });

        if key_comma && !crate::DUMP_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
            crate::DUMP_REQUESTED.store(true, std::sync::atomic::Ordering::Relaxed);
        } else if !key_comma {
            crate::DUMP_REQUESTED.store(false, std::sync::atomic::Ordering::Relaxed);
        }

        if key_semicolon && !crate::MEMORY_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
            crate::MEMORY_REQUESTED.store(true, std::sync::atomic::Ordering::Relaxed);
        } else if !key_semicolon {
            crate::MEMORY_REQUESTED.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }

    // Video

    core.machine.z80.io.colors_at_frame_start = core.machine.z80.io.colors;
    core.machine.z80.io.color_events.clear();

    crate::machine::video::render_frame(
        &core.machine.z80.io.mem,
        &core.machine.z80.io.colors_at_frame_start,
        &core.machine.z80.io.color_events,
        core.machine.z80.io.horcb,
        core.machine.z80.io.verbl,
        CYCLES_PER_FRAME,
        &core.machine.palette,
        &mut core.machine.frame_buffer,
    );

    if let Some(cb) = unsafe { crate::VIDEO_REFRESH_CALLBACK } {
        unsafe {
            cb(
                core.machine.frame_buffer.as_ptr() as *const std::ffi::c_void,
                160,
                102,
                160 * 4,
            )
        };
    }

    if crate::DUMP_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("=== FRAME DUMP ===");
        eprintln!("colors:  {:02x?}", core.machine.z80.io.colors);
        eprintln!("horcb:   {:#04x} (boundary pixel {})", 
            core.machine.z80.io.horcb,
            (core.machine.z80.io.horcb as usize & 0x3f) * 4);
        eprintln!("verbl:   {:#04x} (lines {})", 
            core.machine.z80.io.verbl,
            core.machine.z80.io.verbl / 2);
        eprintln!("magic:   {:#04x}", core.machine.z80.io.magic);
        eprintln!("sound:   {:02x?}", core.machine.z80.io.sound_reg);
        eprintln!("color_events count: {}", core.machine.z80.io.color_events.len());
        for (step, reg, val) in core.machine.z80.io.color_events.iter() {
            eprintln!("  fstep={:>6} reg={} val={:#04x}", step, reg, val);
        }
        eprintln!("--- VRAM $4000-$4FFF (unpacked pixels 0-3) ---");
        for row in 0..102usize {
            let offset = 0x4000 + row * 40;
            let mut pixels = String::new();
            for byte in &core.machine.z80.io.mem[offset..offset+40] {
                pixels.push((((byte >> 6) & 0x03) + b'0') as char);
                pixels.push((((byte >> 4) & 0x03) + b'0') as char);
                pixels.push((((byte >> 2) & 0x03) + b'0') as char);
                pixels.push(((byte & 0x03) + b'0') as char);
            }
            eprintln!("row {:>3}: {}", row, pixels);
        }
        eprintln!("--- FRAMEBUFFER ---");
        let mut color_map: Vec<u32> = Vec::new();
        let chars: Vec<char> = (33u8..=126u8).map(|c| c as char).collect();
        // let chars = ['·', '░', '▒', '▓', '█', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
        for row in 0..102usize {
            let offset = row * 160;
            let mut line = String::new();
            for &pixel in &core.machine.frame_buffer[offset..offset+160] {
                let idx = if let Some(i) = color_map.iter().position(|&c| c == pixel) {
                    i
                } else {
                    color_map.push(pixel);
                    color_map.len() - 1
                };
                line.push(chars[idx.min(chars.len() - 1)]);
            }
            eprintln!("row {:>3}: {}", row, line);
        }
        eprintln!("color legend:");
        for (i, &color) in color_map.iter().enumerate() {
            eprintln!("  {} = #{:06x}", chars[i], color);
        }
        eprintln!("=== END DUMP ===");
    }

    if crate::MEMORY_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("bytes at $022C0 .. $022D9: {:02x?}", &core.machine.z80.io.mem[0x22C0..=0x22D9]);
        eprintln!("byte at $04F3C: {:02x?}", &core.machine.z80.io.mem[0x4F3C]);
        eprintln!("bytes at $02AF5 .. $02B20: {:02x?}", &core.machine.z80.io.mem[0x2AF5..=0x2B20]);
        eprintln!("byte at $04F27: {:02x?}", &core.machine.z80.io.mem[0x4F27]);
    }

    // Machine
    //
    // The Astrocade screen is 262 total scanlines at 455 clocks each.
    // 22 scanlines are vertical blanking (VERT_OFFSET) before the visible
    // area.  INLIN holds the Astrocade scanline number (0-based from the
    // top of the visible area), so the physical scanline is inlin + 22.
    // INMOD bit 3 enables the scanline interrupt.  INFBK is the IRQ vector.
    // The Astrocade's scanline interrupt registers (INFBK, INMOD, INLIN) are
    // written by the game during execution, so they must be read fresh each
    // cycle rather than captured before the loop.  Games typically configure
    // them during their init sequence on the very first frame.
    // Screen geometry: 455 pixel clocks/scanline, pixel clock = CPU clock * 4.
    // CPU cycles/scanline = 455/4 = 113.75 (not integer).
    // Use x4 fixed-point units throughout: 1 x4-unit = 0.25 CPU cycles.
    const TOTAL_SCANLINES: u32 = 262;
    const VERT_OFFSET: u32 = 22;
    const CYCLES_PER_SCANLINE_X4: u32 = 455;                              // exact
    const CYCLES_PER_FRAME_X4: u32 = CYCLES_PER_SCANLINE_X4 * TOTAL_SCANLINES; // 119210

    // Track whether we have a pending assert-mode IRQ that needs clearing.
    // We use assert_irq (not pulse_irq) so the interrupt isn't missed if iff1
    // happens to be false at the exact fire cycle.  But we must clear it on
    // the very next step — otherwise the ASSERT bit stays set and the CPU
    // re-takes the interrupt every time RETI re-enables iff1.
    // last_irq_fire_x4 tracks the most recent scanline cycle at which we fired
    // an IRQ this frame.  Games use raster interrupts — the ISR fires, does work,
    // writes a new inlin value, and returns expecting another IRQ at the new
    // scanline.  We fire again whenever the current irq_fire_cycle_x4 is strictly
    // greater than the last one fired, meaning inlin has advanced forward.
    let mut last_irq_fire_x4: Option<u32> = None;
    let mut irq_asserted = false;

    // frame_step_x4 tracks elapsed time in units of (CPU cycles * 4).
    // This matches the screen geometry exactly: 455 pixel clocks per scanline,
    // with the pixel clock running at 4x the CPU clock, so each scanline is
    // exactly 455 x4-units = 113.75 CPU cycles.  Using integer x4 units avoids
    // the 223-cycle-per-frame rounding error from integer CYCLES_PER_SCANLINE.
    // z80.step() returns CPU cycles; multiply by 4 before accumulating.
    let mut frame_step_x4: u32 = 0;

    while frame_step_x4 < CYCLES_PER_FRAME_X4 {
        // Read interrupt registers fresh — games write them mid-frame.
        let scanline_irq_enabled = (core.machine.z80.io.inmod & 0x08) != 0;
        let irq_physical_line    = (core.machine.z80.io.inlin as u32).saturating_add(VERT_OFFSET);
        let irq_fire_cycle_x4    = CYCLES_PER_SCANLINE_X4 * irq_physical_line;

        // Fire an IRQ when we reach the target scanline.  Support raster interrupts
        // by allowing multiple IRQs per frame: fire again whenever irq_fire_cycle_x4
        // has moved strictly forward past the last position we fired at.
        let is_new_threshold = match last_irq_fire_x4 {
            None => true,
            Some(last) => irq_fire_cycle_x4 > last,
        };
        if scanline_irq_enabled && is_new_threshold && frame_step_x4 >= irq_fire_cycle_x4 {
            core.machine.z80.assert_irq(core.machine.z80.io.infbk);
            irq_asserted = true;
            last_irq_fire_x4 = Some(irq_fire_cycle_x4);
        }

        if scanline_irq_enabled
            && is_new_threshold
            && frame_step_x4 >= irq_fire_cycle_x4
        {
            core.machine.z80.assert_irq(core.machine.z80.io.infbk);
            irq_asserted = true;
            last_irq_fire_x4 = Some(irq_fire_cycle_x4);
            
            #[cfg(feature = "debug_logging")]
            debug_print!(
                core.step_count,
                core.frame_count,
                frame_step_x4 / 4,
                "I pc={:#06x}:{:12} inmod={:#04x} inlin={:#04x} infbk={:#04x} irq_enabled={:5} $4FCE={:#06x} $4FD0={:#06x} $4FD4={:#04x} $4FEA={:#04x} $4FF9={:#04x} scanline={:#03} {}{}",
                core.machine.z80.pc,
                disassemble_at(&core.machine.z80.io.mem, core.machine.z80.pc),
                core.machine.z80.io.inmod,
                core.machine.z80.io.inlin,
                core.machine.z80.io.infbk,
                (core.machine.z80.io.inmod & 0x08) != 0,
                u16::from_le_bytes([
                    core.machine.z80.io.mem[0x4FCE],
                    core.machine.z80.io.mem[0x4FCF]
                ]),
                u16::from_le_bytes([
                    core.machine.z80.io.mem[0x4FD0],
                    core.machine.z80.io.mem[0x4FD1]
                ]),
                core.machine.z80.io.mem[0x4FD4],
                core.machine.z80.io.mem[0x4FEA],
                core.machine.z80.io.mem[0x4FF9],
                irq_physical_line,
                if catch_trigger { "Trigger" } else { "" },
                if catch_enter { "Enter" } else { "" },
            );
        }

        // DEBUG: Per-second state dump
        #[cfg(feature = "debug_logging")]
        if core.frame_count % 60 == 0 && frame_step_x4 == 0 {
            debug_print!(
                core.step_count,
                core.frame_count,
                frame_step_x4 / 4,
                "> pc={:#06x}:{:12} inmod={:#04x} inlin={:#04x} infbk={:#04x} irq_enabled={:5} $4FCE={:#06x} $4FD0={:#06x} $4FD4={:#04x} $4FEA={:#04x} $4FF9={:#04x} scanline={:#03} {}{}",
                core.machine.z80.pc,
                disassemble_at(&core.machine.z80.io.mem, core.machine.z80.pc),
                core.machine.z80.io.inmod,
                core.machine.z80.io.inlin,
                core.machine.z80.io.infbk,
                (core.machine.z80.io.inmod & 0x08) != 0,
                u16::from_le_bytes([
                    core.machine.z80.io.mem[0x4FCE],
                    core.machine.z80.io.mem[0x4FCF]
                ]),
                u16::from_le_bytes([
                    core.machine.z80.io.mem[0x4FD0],
                    core.machine.z80.io.mem[0x4FD1]
                ]),
                core.machine.z80.io.mem[0x4FD4],
                core.machine.z80.io.mem[0x4FEA],
                core.machine.z80.io.mem[0x4FF9],
                irq_physical_line,
                if catch_trigger { "Trigger" } else { "" },
                if catch_enter { "Enter" } else { "" },
            );
        }

        // current_frame_step is used by flush_audio and color_events as a
        // proportion of CYCLES_PER_FRAME.  Convert from x4 units.
        core.machine.z80.io.current_frame_step = frame_step_x4 / 4;
        core.machine.z80.io.frame_count = core.frame_count;
        core.machine.z80.io.step_count = core.step_count;
        let current_inmod = core.machine.z80.io.inmod;
        let current_inlin = core.machine.z80.io.inlin;
        let current_infbk = core.machine.z80.io.infbk;
        let cycles = core.machine.z80.step();
        core.step_count += 1;
        frame_step_x4 += cycles * 4;

        // DEBUG: diagnostic
        #[cfg(feature = "debug_logging")]
        if (current_inmod != core.machine.z80.io.inmod) || (current_inlin != core.machine.z80.io.inlin) || (current_infbk != core.machine.z80.io.infbk) {
            debug_print!(
                core.step_count,
                core.frame_count,
                frame_step_x4 / 4,
                "» pc={:#06x}:{:12} inmod={:#04x} inlin={:#04x} infbk={:#04x} irq_enabled={:5} $4FCE={:#06x} $4FD0={:#06x} $4FD4={:#04x} $4FEA={:#04x} $4FF9={:#04x} scanline={:#03} {}{}",
                core.machine.z80.pc,
                disassemble_at(&core.machine.z80.io.mem, core.machine.z80.pc),
                core.machine.z80.io.inmod,
                core.machine.z80.io.inlin,
                core.machine.z80.io.infbk,
                (core.machine.z80.io.inmod & 0x08) != 0,
                u16::from_le_bytes([
                    core.machine.z80.io.mem[0x4FCE],
                    core.machine.z80.io.mem[0x4FCF]
                ]),
                u16::from_le_bytes([
                    core.machine.z80.io.mem[0x4FD0],
                    core.machine.z80.io.mem[0x4FD1]
                ]),
                core.machine.z80.io.mem[0x4FD4],
                core.machine.z80.io.mem[0x4FEA],
                core.machine.z80.io.mem[0x4FF9],
                irq_physical_line,
                if catch_trigger { "Trigger" } else { "" },
                if catch_enter { "Enter" } else { "" },
            );
        }

        // Clear a held IRQ in the same iteration it was asserted, after step().
        // The CPU has had exactly one instruction opportunity to take it.
        if irq_asserted {
            core.machine.z80.clr_irq();
            irq_asserted = false;
        }
    }

    core.frame_count += 1;

    // Audio

    {
        let io = &mut core.machine.z80.io;
        io.flush_audio();

        let frames_per_frame = 800usize;
        let total_samples = frames_per_frame * 2;
        let mut audio_buffer = vec![0i16; total_samples];

        io.audio_buffer.resize(frames_per_frame, 0);

        for i in 0..frames_per_frame {
            audio_buffer[i * 2]     = io.audio_buffer[i];
            audio_buffer[i * 2 + 1] = io.audio_buffer[i];
        }

        io.audio_buffer.clear();

        if let Some(cb) = unsafe { crate::AUDIO_SAMPLE_BATCH_CALLBACK } {
            unsafe { cb(audio_buffer.as_ptr(), frames_per_frame as usize) };
        }
    }

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
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize_size() -> usize {
    crate::savestate::SAVE_STATE_SIZE
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize(data: *mut std::ffi::c_void, size: usize) -> bool {
    if data.is_null() || size < crate::savestate::SAVE_STATE_SIZE {
        return false;
    }
    let core_guard = crate::CORE.lock().unwrap();
    let core = match core_guard.as_ref() {
        Some(c) => c,
        None => return false,
    };
    let ss = crate::savestate::serialize(core);
    unsafe {
        std::ptr::copy_nonoverlapping(
            &ss as *const crate::savestate::SaveState as *const u8,
            data as *mut u8,
            crate::savestate::SAVE_STATE_SIZE,
        );
    }
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unserialize(data: *const std::ffi::c_void, size: usize) -> bool {
    if data.is_null() || size < crate::savestate::SAVE_STATE_SIZE {
        return false;
    }
    let mut ss = std::mem::MaybeUninit::<crate::savestate::SaveState>::uninit();
    unsafe {
        std::ptr::copy_nonoverlapping(
            data as *const u8,
            ss.as_mut_ptr() as *mut u8,
            crate::savestate::SAVE_STATE_SIZE,
        );
    }
    let ss = unsafe { ss.assume_init() };
    let mut core_guard = crate::CORE.lock().unwrap();
    let core = match core_guard.as_mut() {
        Some(c) => c,
        None => return false,
    };
    crate::savestate::unserialize(core, &ss)
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_data(id: u32) -> *mut std::ffi::c_void {
    const RETRO_MEMORY_SYSTEM_RAM: u32 = 2;
    if id != RETRO_MEMORY_SYSTEM_RAM {
        return std::ptr::null_mut();
    }
    let mut core_guard = crate::CORE.lock().unwrap();
    match core_guard.as_mut() {
        Some(core) => core.machine.z80.io.mem.as_mut_ptr() as *mut std::ffi::c_void,
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_size(id: u32) -> usize {
    const RETRO_MEMORY_SYSTEM_RAM: u32 = 2;
    if id == RETRO_MEMORY_SYSTEM_RAM { 0x10000 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_reset() {
    eprintln!("retro_cheat_reset(): started");
    eprintln!("retro_cheat_reset(): finished");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_set(_index: u32, _enabledd: bool, _code: *const std::ffi::c_char) {
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
    _info: *const RetroGameInfo,
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
            let mut retro_msg = RetroMessage {
                msg: c_msg.as_ptr(),
                frames: 600,
            };
            cb(
                RETRO_ENVIRONMENT_SET_MESSAGE,
                &mut retro_msg as *mut RetroMessage as *mut std::ffi::c_void,
            );
        }
    }
}

#[cfg(feature = "debug_logging")]
fn disassemble_at(mem: &[u8; 0x10000], pc: u16) -> String {
    let pc = pc as usize;
    let op = mem[pc];
    let b1 = if pc + 1 < 0x10000 { mem[pc + 1] } else { 0 };
    let b2 = if pc + 2 < 0x10000 { mem[pc + 2] } else { 0 };
    let b3 = if pc + 3 < 0x10000 { mem[pc + 3] } else { 0 };
    match op {
        0x00 => "NOP".to_string(),
        0x01 => format!("LD BC, ${:04x}", u16::from_le_bytes([b1, b2])),
        0x11 => format!("LD DE, ${:04x}", u16::from_le_bytes([b1, b2])),
        0x21 => format!("LD HL, ${:04x}", u16::from_le_bytes([b1, b2])),
        0x31 => format!("LD SP, ${:04x}", u16::from_le_bytes([b1, b2])),
        0xC3 => format!("JP ${:04x}", u16::from_le_bytes([b1, b2])),
        0xCD => format!("CALL ${:04x}", u16::from_le_bytes([b1, b2])),
        0xC9 => "RET".to_string(),
        0xD3 => format!("OUT (${:02x}), A", b1),
        0xDB => format!("IN A, (${:02x})", b1),
        0xED => match b1 {
            0x43 => format!("LD (${:04x}), BC", u16::from_le_bytes([b2, b3])),
            0x53 => format!("LD (${:04x}), DE", u16::from_le_bytes([b2, b3])),
            0x63 => format!("LD (${:04x}), HL", u16::from_le_bytes([b2, b3])),
            0x73 => format!("LD (${:04x}), SP", u16::from_le_bytes([b2, b3])),
            0x79 => "OUT (C), A".to_string(),
            0xB3 => "OTIR".to_string(),
            0xB9 => "CPDR".to_string(),
            _ => format!("ED ${:02x}", b1),
        },
        0xF3 => "DI".to_string(),
        0xFB => "EI".to_string(),
        0xFF => "RST $38".to_string(),
        0xC0 => "RET NZ".to_string(),
        0xC8 => "RET Z".to_string(),
        0xD0 => "RET NC".to_string(),
        0xD8 => "RET C".to_string(),
        0x18 => format!("JR ${:04x}", (pc as i32 + 2 + b1 as i8 as i32) as u16),
        0x20 => format!("JR NZ, ${:04x}", (pc as i32 + 2 + b1 as i8 as i32) as u16),
        0x28 => format!("JR Z, ${:04x}", (pc as i32 + 2 + b1 as i8 as i32) as u16),
        _ => format!("${:02x}", op),
    }
}