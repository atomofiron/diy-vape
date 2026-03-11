use crate::types::{DeciOhm, MilliSeconds, Ohm, Progress, Seconds, Time};
use core::ops::Range;

pub const SCREEN_WIDTH: u32 = 128;
pub const SCREEN_HEIGHT: u32 = 64;

pub const LIMIT_RANGE: Range<Seconds> = 1..5;
pub const RESISTANCE_RANGE: Range<DeciOhm> = 1..255;

pub const PROGRESS_STEP: Progress = 3;
pub const PROGRESS_MIN: Progress = 0;
pub const PROGRESS_MAX: Progress = 255;
pub const SECOND: Time = 1000;
pub const PROGRESS_WIDTH: u32 = (PROGRESS_MAX / PROGRESS_STEP) as u32 + 1;
pub const PROGRESS_OFFSET: i32 = (SCREEN_WIDTH - PROGRESS_WIDTH) as i32 / 2;
pub const BATTERY_PERIOD: MilliSeconds = 60000;
pub const SCREENSAVER_TIMEOUT: MilliSeconds = 10000;
pub const IDLE_PERIOD: MilliSeconds = 10;
pub const SLEEP_PERIOD: MilliSeconds = 1000;
pub const MV: f32 = 1000.0;
pub const VOLTS_MAX: f32 = 4.2;
pub const VOLTS_MIN: f32 = 3.5;
pub const SYSTEM_RESISTANCE: Ohm = 0.04;

pub const STORAGE_RANGE: Range<u32> = 0..0x20000;
