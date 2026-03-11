use crate::flash::savable::Savable;
use crate::types::DeciSecond;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MaxSize, Clone, Debug)]
pub struct Stats {
    pub total: DeciSecond,
    pub count: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { total: 0, count: 0 }
    }
}

impl Savable for Stats {
    const FLASH_KEY: u8 = 2;
}
