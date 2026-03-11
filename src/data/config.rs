use crate::data::power::Power;
use crate::flash::savable::Savable;
use crate::types::{DeciOhm, Second};
use crate::util::round;
use crate::values::SYSTEM_RESISTANCE;
use libm::powf;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MaxSize, Clone, Debug)]
pub struct Config {
    pub power: Power,
    pub limit: Second,
    pub resistance: DeciOhm,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            power: Power::Medium,
            limit: 3,
            resistance: 12,
        }
    }
}

impl Savable for Config {
    const FLASH_KEY: u8 = 1;
}

impl Config {

    pub fn watts(&self, volts: f32) -> u8 {
        let load = self.resistance as f32 / 10f32;
        let efficiency = load / (load + SYSTEM_RESISTANCE);
        let real_volts = volts * efficiency;
        let real_power = powf(real_volts, 2.0) / load;
        return match round(real_power) {
            w if w < 0 => 0,
            w if w > 255 => 255,
            w => w as u8,
        }
    }
}
