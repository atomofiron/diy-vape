use nrf52840_hal::gpio::{Output, Pin, PushPull};
use nrf52840_hal::pac::TWIM0;
use nrf52840_hal::Twim;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::{DisplaySize128x64, I2CInterface};
use ssd1306::Ssd1306;

pub type Display = Ssd1306<I2CInterface<Twim<TWIM0>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;
pub type POPP = Pin<Output<PushPull>>;
pub type DeciOhms = u8;
pub type Seconds = u8;
pub type DeciSeconds = u32;
