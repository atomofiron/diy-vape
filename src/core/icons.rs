use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::BinaryColor;
use crate::icon_bytes;

pub const BRIGHTNESS_ICON_SIZE: u32 = 12;

const SUN_STROKE_BYTES: [u8; 24] = icon_bytes!("
░░░█░░░░█░░░
░░░██░░██░░░
░░░░░░░░░░░░
██░░░██░░░██
░█░░█░░█░░█░
░░░█░░░░█░░░
░░░█░░░░█░░░
░█░░█░░█░░█░
██░░░██░░░██
░░░░░░░░░░░░
░░░██░░██░░░
░░░█░░░░█░░░
");
pub const SUN_STROKE: ImageRaw<BinaryColor> = ImageRaw::<BinaryColor>::new(&SUN_STROKE_BYTES, BRIGHTNESS_ICON_SIZE);

const SUN_FILL_BYTES: [u8; 24] = icon_bytes!("
░░░█░░░░█░░░
░░░██░░██░░░
░░░░░░░░░░░░
██░░░██░░░██
░█░░████░░█░
░░░██████░░░
░░░██████░░░
░█░░████░░█░
██░░░██░░░██
░░░░░░░░░░░░
░░░██░░██░░░
░░░█░░░░█░░░
");
pub const SUN_FILL: ImageRaw<BinaryColor> = ImageRaw::<BinaryColor>::new(&SUN_FILL_BYTES, BRIGHTNESS_ICON_SIZE);

const MOON_STROKE_BYTES: [u8; 24] = icon_bytes!("
░░░░░░░░░░░░
░░░░████░░░░
░░░█░░█░░░░░
░░█░░█░░░░░░
░█░░█░░░░░░░
░█░░█░░░░░░░
░█░░░█░░░░█░
░█░░░░█░░██░
░░█░░░░███░░
░░░█░░░░█░░░
░░░░████░░░░
░░░░░░░░░░░░
");
pub const MOON_STROKE: ImageRaw<BinaryColor> = ImageRaw::<BinaryColor>::new(&MOON_STROKE_BYTES, BRIGHTNESS_ICON_SIZE);

const MOON_FILL_BYTES: [u8; 24] = icon_bytes!("
░░░░░░░░░░░░
░░░░████░░░░
░░░████░░░░░
░░████░░░░░░
░████░░░░░░░
░████░░░░░░░
░█████░░░░█░
░██████░░██░
░░████████░░
░░░██████░░░
░░░░████░░░░
░░░░░░░░░░░░
");
pub const MOON_FILL: ImageRaw<BinaryColor> = ImageRaw::<BinaryColor>::new(&MOON_FILL_BYTES, BRIGHTNESS_ICON_SIZE);
