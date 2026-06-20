use std::sync::Mutex;

use crate::core::AstrocadeCore;

pub mod core;
pub mod libretro;
pub mod machine;
pub mod savestate;
pub mod types;

// Astrocade timing constants
pub const ASTROCADE_CLOCK: u32 = 1_789_772;  // Z80 and sound chip clock (14_318_181 / 8)
pub const CYCLES_PER_FRAME: u32 = ASTROCADE_CLOCK / 60;  // ~29_829
pub const SAMPLE_RATE: u32 = 48_000;
pub const SAMPLES_PER_FRAME: u32 = SAMPLE_RATE / 60;     // 800

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

pub static LOG_CALLBACK: Mutex<
    Option<unsafe extern "C" fn(crate::types::RetroLogLevel, *const std::ffi::c_char)>,
> = Mutex::new(None);

pub static DUMP_REQUESTED: std::sync::atomic::AtomicBool = 
    std::sync::atomic::AtomicBool::new(false);

pub static MEMORY_REQUESTED: std::sync::atomic::AtomicBool = 
    std::sync::atomic::AtomicBool::new(false);

#[macro_export]
macro_rules! retro_log {
    ($level:expr, $($arg:tt)*) => {
        if let Some(cb) = *crate::LOG_CALLBACK.lock().unwrap() {
            let msg = std::ffi::CString::new(format!("{}\n", format!($($arg)*))).unwrap();
                unsafe { cb($level, msg.as_ptr()) };
        }
    }
}

#[macro_export]
macro_rules! debug_print {
    ($step:expr, $frame:expr, $fstep:expr, $($arg:tt)*) => {
        #[cfg(feature = "debug_logging")]
        eprintln!(
            "step={:>12} frame={:>6} fstep={:>6} | {}",
            $step,
            $frame,
            $fstep,
            format!($($arg)*)
        );
    }
}