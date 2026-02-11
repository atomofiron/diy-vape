use crate::data::power::Power;
use crate::types::{DeciOhms, Seconds};

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