use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};
use crate::data::power::Power;
use crate::flash::savable::Savable;
use crate::types::{DeciOhms, Seconds};

#[derive(Serialize, Deserialize, MaxSize, Clone, Debug)]
pub struct Config {
    pub power: Power,
    pub limit: Seconds,
    pub resistance: DeciOhms,
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
