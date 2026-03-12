use crate::data::power::Power;
use crate::flash::savable::Savable;
use crate::types::{Brightness, DeciOhm, MilliVolt, MilliWatt, Second};
use crate::values::{BRIGHTNESS_RANGE, BRIGHTNESS_RANGE_RAW};
use core::cmp::max;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MaxSize, Clone, Debug)]
pub struct Config {
    pub power: Power,
    pub limit: Second,
    pub resistance: DeciOhm,
    pub brightness: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            power: Power::Medium,
            limit: 3,
            resistance: 12,
            brightness: 2,
        }
    }
}

impl Savable for Config {
    const FLASH_KEY: u8 = 1;
}

impl Config {

    pub fn milliwatts(&self, mv: MilliVolt) -> MilliWatt {
        let mv = mv as MilliWatt;
        let load = self.resistance as MilliWatt;
        return mv.pow(2) / load / 100;
    }

    pub fn brightness(&self) -> Brightness {
        let level = self.brightness as u16;
        let level_max = BRIGHTNESS_RANGE.end as u16;
        let raw_max = BRIGHTNESS_RANGE_RAW.end as u16;
        let raw = (raw_max * level / level_max) as Brightness;
        return max(raw, BRIGHTNESS_RANGE_RAW.start)
    }
}
