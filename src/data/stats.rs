use crate::flash::savable::{FlashKey, Savable};
use crate::types::{DeciSecond, PuffCount};
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MaxSize, Default, Clone, Debug, PartialEq)]
pub struct Stats {
    pub coil: DeciSecond,
    pub total: DeciSecond,
    pub count: PuffCount,
}

impl Stats {

    pub fn reset_coil(&mut self) {
        self.coil = 0;
    }

    pub fn reset_count(&mut self) {
        self.count = 0;
    }

    pub fn reset_total(&mut self) {
        self.total = 0;
    }
}

impl Savable for Stats {
    const FLASH_KEY: FlashKey = 2;
}
