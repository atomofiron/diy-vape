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

impl Savable for Stats {
    const FLASH_KEY: FlashKey = 2;
}
