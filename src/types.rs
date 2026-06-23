use crate::ext::error::ErrorMessage;
use crate::flash::flash::AsyncFlash;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Styled};
use nrf52840_hal::gpio::{Input, Output, Pin};
use nrf52840_hal::pac::TWIM0;
use nrf52840_hal::Twim;
use sequential_storage::cache::NoCache;
use sequential_storage::map::MapStorage;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::{DisplaySize128x64, I2CInterface};
use ssd1306::Ssd1306;

pub type Icon<'i,'r> = Image<'i, IconRaw<'r>>;
pub type IconRaw<'r> = ImageRaw<'r, BinaryColor>;
pub type TextStyle<'l> = MonoTextStyle<'l, BinaryColor>;
pub type FigureStyle = PrimitiveStyle<BinaryColor>;
pub type StyledLine = Styled<Line, FigureStyle>;

pub type Rslt<T> = Result<T, ErrorMessage>;
pub type Display = Ssd1306<I2CInterface<Twim<TWIM0>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;
pub type Storage = MapStorage<u8, AsyncFlash, NoCache>;
pub type PinIn<M> = Pin<Input<M>>;
pub type PinOut<M> = Pin<Output<M>>;
pub type DeciOhm = u8;
pub type Ohm = f32;
pub type Second = u8;
pub type DeciSecond = u32;
pub type MilliSecond = u64;
pub type MilliWatt = u32;
pub type MilliVolt = u16;
pub type Percent = u8;
pub type Duty = u16;
pub type Progress = u8;
pub type Brightness = u8;
pub type Time = u64; // ms
