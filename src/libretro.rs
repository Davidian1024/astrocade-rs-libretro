
use crate::{core::AstrocadeCore, types::RetroSystemInfo};

#[unsafe(no_mangle)]
pub extern "C" fn retro_api_version() -> u32 {
    eprintln!("retro_api_version(): started\n");
    eprintln!("retro_api_version(): finished\n");
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_info(info: *mut RetroSystemInfo) {
    eprintln!("retro_get_system_info(): started\n");
    unsafe {
        (*info).library_name = concat!(env!("CARGO_PKG_NAME"), "\0").as_ptr() as *const std::ffi::c_char;
        (*info).library_version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_BUILD_TIMESTAMP"), "\0").as_ptr() as *const std::ffi::c_char;
        (*info).valid_extensions = c"".as_ptr();
        (*info).need_fullpath = false;
        (*info).block_extract = false;
    }
    eprintln!("retro_get_system_info(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_av_info(info: *mut crate::types::RetroSystemAvInfo) {
    eprintln!("retro_get_system_av_info(): started\n");
    unsafe {
        (*info).geometry.base_width = 160;
        (*info).geometry.base_height = 102;
        (*info).geometry.max_width = 160;
        (*info).geometry.max_height = 102;
        (*info).geometry.aspect_ratio = 0.0;
        (*info).timing.fps = 60.0;
        (*info).timing.sample_rate = 48000.0;
    }
    eprintln!("retro_get_system_av_info(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_environment(
    cb: unsafe extern "C" fn(u32, *mut std::ffi::c_void) -> bool,
) {
    eprintln!("retro_set_environment(): started\n");
    unsafe {
        crate::ENVIRONMENT_CALLBACK = Some(cb);

        let mut supports_no_game = true;
        cb(
            crate::types::RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
            &mut supports_no_game as *mut bool as *mut std::ffi::c_void,
        );
    }
    eprintln!("retro_set_environment(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_video_refresh(
    cb: unsafe extern "C" fn(*const std::ffi::c_void, u32, u32, usize),
) {
    eprintln!("retro_set_video_refresh(): started\n");
    unsafe {
        crate::VIDEO_REFRESH_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_video_refresh(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample(cb: unsafe extern "C" fn(i16, i16)) {
    eprintln!("retro_set_audio_sample(): started\n");
    unsafe {
        crate::AUDIO_SAMPLE_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_audio_sample(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample_batch(
    cb: unsafe extern "C" fn(*const i16, usize) -> usize,
) {
    eprintln!("retro_set_audio_sample_batch(): started\n");
    unsafe {
        crate::AUDIO_SAMPLE_BATCH_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_audio_sample_batch(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_poll(cb: unsafe extern "C" fn()) {
    eprintln!("retro_set_input_poll(): started\n");
    unsafe {
        crate::INPUT_POLL_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_input_poll(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_state(cb: unsafe extern "C" fn(u32, u32, u32, u32) -> i16) {
    eprintln!("retro_set_input_state(): started\n");
    unsafe {
        crate::INPUT_STATE_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_input_state(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_init() {
    eprintln!("retro_init(): started\n");
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
    eprintln!("retro_init(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game(_game: *const crate::types::RetroGameInfo) -> bool {
    eprintln!("retro_load_game(): started\n");
    eprintln!("retro_load_game(): finished\n");
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_run() {
    // eprintln!("retro_run(): started\n");

    // Input

    if let Some(poll) = unsafe { crate::INPUT_POLL_CALLBACK } {
        unsafe { poll() };
    }

    // Video

    const WIDTH: u32 = 160;
    const HEIGHT: u32 = 102;
    const PITCH: usize = (WIDTH as usize) * 4;

    let buffer = vec![0u32; (WIDTH * HEIGHT) as usize];

    if let Some(cb) = unsafe { crate::VIDEO_REFRESH_CALLBACK } {
        unsafe { cb(
            buffer.as_ptr() as *const std::ffi::c_void,
            WIDTH,
            HEIGHT,
            PITCH,
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

    let mut core = crate::CORE.lock().unwrap();
    let core = core.as_mut().unwrap();

    core.machine.z80.step();

    // eprintln!("retro_run(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_deinit() {
    eprintln!("retro_deinit(): started\n");
    let mut core = crate::CORE.lock().unwrap();
    *core = None;
    eprintln!("retro_deinit(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unload_game() {
    eprintln!("retro_unload_game(): started\n");
    eprintln!("retro_unload_game(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_region() -> u32 {
    eprintln!("retro_get_region(): started\n");
    eprintln!("retro_get_region(): finished\n");
    0 // NTSC
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize_size() -> usize {
    eprintln!("retro_serialize_size(): started\n");
    eprintln!("retro_serialize_size(): finished\n");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize(_data: *mut std::ffi::c_void, _size: usize) -> bool {
    eprintln!("retro_serialize(): started\n");
    eprintln!("retro_serialize(): finished\n");
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unserialize(_data: *const std::ffi::c_void, _size: usize) -> bool {
    eprintln!("retro_unserialize(): started\n");
    eprintln!("retro_unserialize(): finished\n");
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_data(_id: u32) -> *mut std::ffi::c_void {
    eprintln!("retro_get_memory_data(): started\n");
    eprintln!("retro_get_memory_data(): finished\n");
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_size(_id: u32) -> usize {
    eprintln!("retro_get_memory_size(): started\n");
    eprintln!("retro_get_memory_size(): finished\n");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_reset() {
    eprintln!("retro_cheat_reset(): started\n");
    eprintln!("retro_cheat_reset(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_set(_index: u32,_enabledd: bool, _code: *const std::ffi::c_char) {
    eprintln!("retro_cheat_set(): started\n");
    eprintln!("retro_cheat_set(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_controller_port_device(_port: u32, _device: u32) {
    eprintln!("retro_set_controller_port_device(): started\n");
    eprintln!("retro_set_controller_port_device(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_reset() {
    eprintln!("retro_reset(): started\n");
    eprintln!("retro_reset(): finished\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game_special(
    _game_type: u32,
    _info: *const crate::types::RetroGameInfo,
    _num_info: usize,
) -> bool {
    eprintln!("retro_load_game_special(): started\n");
    eprintln!("retro_load_game_special(): finished\n");
    false
}
