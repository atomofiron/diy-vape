use core::ops::Range;
use crate::types::{DeciOhms, Seconds};

pub const SCREEN_WIDTH: u32 = 128;
pub const SCREEN_HEIGHT: u32 = 64;

pub const LIMIT_RANGE: Range<Seconds> = 1..5;
pub const RESISTANCE_RANGE: Range<DeciOhms> = 1..255;

pub const PROGRESS_STEP: u32 = 3;
pub const PROGRESS_WIDTH: u32 = 255 / PROGRESS_STEP + 1;
pub const PROGRESS_OFFSET: i32 = (SCREEN_WIDTH - PROGRESS_WIDTH) as i32 / 2;
pub const BATTERY_PERIOD: u32 = 60000; // ms
