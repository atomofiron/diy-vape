use core::fmt::Debug;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

pub type FlashKey = u8;

pub trait Savable : Serialize + for<'de> Deserialize<'de> + MaxSize + Debug + Default {
    const FLASH_KEY: FlashKey;
    const FLASH_BUFFER_SIZE: usize = (Self::POSTCARD_MAX_SIZE + size_of::<FlashKey>() + 3) & !3;
}
