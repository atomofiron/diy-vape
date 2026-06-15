use crate::icon_bytes;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::{Polyline, Styled};
use crate::core::graphics::{StyledPolyline, WHITE_STROKE};

// ˟
const CROSS_POINTS: [Point; 5] = [Point::new(0, 0), Point::new(4, 4), Point::new(2, 2), Point::new(0, 4), Point::new(4, 0)];
pub const ICON_CROSS: StyledPolyline = Styled::new(Polyline::new(&CROSS_POINTS), WHITE_STROKE);

// Ω
const OHM_POINTS: [Point; 12] = [Point::new(0, 9), Point::new(2, 9), Point::new(2, 7), Point::new(0, 5), Point::new(0, 3), Point::new(3, 0), Point::new(5, 0), Point::new(8, 3), Point::new(8, 5), Point::new(6, 7), Point::new(6, 9), Point::new(8, 9)];
pub const ICON_OHM: StyledPolyline = Styled::new(Polyline::new(&OHM_POINTS), WHITE_STROKE);

// ⚡ (fill)
const CHARGING_POINTS: [Point; 7] = [Point::new(3, 0), Point::new(0, 7), Point::new(5, 3), Point::new(2, 10), Point::new(2, 4), Point::new(3, 6), Point::new(3, 2)];
pub const ICON_CHARGING: Polyline = Polyline::new(&CHARGING_POINTS);

// ⚡ (stroke)
const CHARGED_POINTS: [Point;8] = [Point::new(3, 0), Point::new(0, 7), Point::new(0, 6), Point::new(1, 5), Point::new(4, 5), Point::new(5, 4), Point::new(5, 3), Point::new(2, 10)];
pub const ICON_CHARGED: Polyline = Polyline::new(&CHARGED_POINTS);

// !
const WARNING_POINTS: [Point; 8] = [Point::new(0, 9), Point::new(0, 10), Point::new(1, 10), Point::new(1, 8), Point::new(0, 7), Point::new(0, 0), Point::new(1, 0), Point::new(1, 6)];
pub const ICON_WARNING: Polyline = Polyline::new(&WARNING_POINTS);

// empty
pub const ICON_EMPTY: Polyline = Polyline::new(&[]);

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
