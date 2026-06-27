use crate::flash::savable::{FlashKey, Savable};
use crate::types::{DeciSecond, PuffCount};
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MaxSize, Clone, Debug, PartialEq)]
pub struct Stats {
    pub total: DeciSecond,
    pub count: PuffCount,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { total: 0, count: 0 }
    }
}

impl Savable for Stats {
    const FLASH_KEY: FlashKey = 2;
}
