use core::ops::Range;
use crate::types::{DeciOhms, MilliSeconds, Seconds};

pub const SCREEN_WIDTH: u32 = 128;
pub const SCREEN_HEIGHT: u32 = 64;

pub const LIMIT_RANGE: Range<Seconds> = 1..5;
pub const RESISTANCE_RANGE: Range<DeciOhms> = 1..255;

pub const PROGRESS_STEP: u32 = 3;
pub const PROGRESS_MAX: u8 = 255;
pub const PROGRESS_WIDTH: u32 = PROGRESS_MAX as u32 / PROGRESS_STEP + 1;
pub const PROGRESS_OFFSET: i32 = (SCREEN_WIDTH - PROGRESS_WIDTH) as i32 / 2;
pub const BATTERY_PERIOD: MilliSeconds = 60000;
pub const SCREENSAVER_TIMEOUT: MilliSeconds = 10000;
pub const IDLE_PERIOD: MilliSeconds = 10;
pub const SLEEP_PERIOD: MilliSeconds = 1000;

pub const STORAGE_RANGE: Range<u32> = 0..0x20000;
