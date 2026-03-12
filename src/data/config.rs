use crate::data::power::Power;
use crate::flash::savable::Savable;
use crate::types::{DeciOhm, MilliVolt, MilliWatt, Second};
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

    pub fn milliwatts(&self, mv: MilliVolt) -> MilliWatt {
        let mv = mv as MilliWatt;
        let load = self.resistance as MilliWatt;
        return mv.pow(2) / load / 100;
    }
}
