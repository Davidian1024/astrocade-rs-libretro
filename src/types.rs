pub const RETRO_PIXEL_FORMAT_XRGB8888: u32 = 1;

pub const RETRO_ENVIRONMENT_SET_MESSAGE: u32 = 6;
pub const RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY: u32 = 9;
pub const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: u32 = 10;
pub const RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: u32 = 18;
pub const RETRO_ENVIRONMENT_SET_KEYBOARD_REPORTING: u32 = 26;
pub const RETRO_ENVIRONMENT_GET_LOG_INTERFACE: u32 = 27;

pub const RETRO_DEVICE_ID_JOYPAD_B:      u32 = 0;
pub const RETRO_DEVICE_ID_JOYPAD_UP:     u32 = 4;
pub const RETRO_DEVICE_ID_JOYPAD_DOWN:   u32 = 5;
pub const RETRO_DEVICE_ID_JOYPAD_LEFT:   u32 = 6;
pub const RETRO_DEVICE_ID_JOYPAD_RIGHT:  u32 = 7;

pub const RETRO_DEVICE_KEYBOARD: u32 = 3;
pub const RETRO_DEVICE_ID_KEYBOARD_1: u32 = 49;
pub const RETRO_DEVICE_ID_KEYBOARD_2: u32 = 50;
pub const RETRO_DEVICE_ID_KEYBOARD_3: u32 = 51;
pub const RETRO_DEVICE_ID_KEYBOARD_4: u32 = 52;
pub const RETRO_DEVICE_ID_KEYBOARD_RETURN: u32 = 13;

#[repr(C)]
pub struct RetroSystemInfo {
    pub library_name: *const std::ffi::c_char,
    pub library_version: *const std::ffi::c_char,
    pub valid_extensions: *const std::ffi::c_char,
    pub need_fullpath: bool,
    pub block_extract: bool,
}

#[repr(C)]
pub struct RetroGameGeometry {
    pub base_width: u32,
    pub base_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub aspect_ratio: f32,
}

#[repr(C)]
pub struct RetroSystemTiming {
    pub fps: f64,
    pub sample_rate: f64,
}

#[repr(C)]
pub struct RetroSystemAvInfo {
    pub geometry: RetroGameGeometry,
    pub timing: RetroSystemTiming,
}

#[repr(C)]
pub struct RetroGameInfo {
    pub path: *const std::ffi::c_char,
    pub data: *const std::ffi::c_void,
    pub size: usize,
    pub meta: *const std::ffi::c_char,
}

#[repr(C)]
pub struct RetroMessage {
    pub msg: *const std::ffi::c_char,
    pub frames: u32,
}

#[repr(u32)]
pub enum RetroLogLevel {
    Debug = 0,
    Info  = 1,
    Warn  = 2,
    Error = 3,
}

#[repr(C)]
pub struct RetroLogCallback {
    pub log: Option<unsafe extern "C" fn(level: RetroLogLevel, fmt: *const std::ffi::c_char)>,
}