use std::sync::Mutex;

use crate::core::AstrocadeCore;

pub mod core;
pub mod libretro;
pub mod machine;
pub mod types;

static CORE: Mutex<Option<AstrocadeCore>> = Mutex::new(None);

pub static mut ENVIRONMENT_CALLBACK: Option<
    unsafe extern "C" fn(u32, *mut std::ffi::c_void) -> bool,
> = None;

pub static mut VIDEO_REFRESH_CALLBACK: Option<
    unsafe extern "C" fn(*const std::ffi::c_void, u32, u32, usize),
> = None;

pub static mut AUDIO_SAMPLE_CALLBACK: Option<unsafe extern "C" fn(i16, i16)> = None;

pub static mut AUDIO_SAMPLE_BATCH_CALLBACK: Option<
    unsafe extern "C" fn(*const i16, usize) -> usize,
> = None;

pub static mut INPUT_POLL_CALLBACK: Option<unsafe extern "C" fn()> = None;

pub static mut INPUT_STATE_CALLBACK: Option<unsafe extern "C" fn(u32, u32, u32, u32) -> i16> = None;

pub static SYSTEM_DIRECTORY: Mutex<Option<String>> = Mutex::new(None);

pub static LOG_CALLBACK: Mutex<Option<unsafe extern "C" fn(crate::types::RetroLogLevel, *const std::ffi::c_char)>> = Mutex::new(None);

#[macro_export]
macro_rules! retro_log {
    ($level:expr, $($arg:tt)*) => {
        if let Some(cb) = *crate::LOG_CALLBACK.lock().unwrap() {
            let msg = std::ffi::CString::new(format!("{}\n", format!($($arg)*))).unwrap();
            unsafe { cb($level, msg.as_ptr()); }
        }
    }
}