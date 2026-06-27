use crate::icon_bytes;
use crate::types::IconRaw;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::BinaryColor;

pub const ICON_EMPTY: IconRaw = ImageRaw::<BinaryColor>::new(&[0u8; 0], 0);

const CROSS_BYTES: [u8; 5] = icon_bytes!("
█░░░█
░█░█░
░░█░░
░█░█░
█░░░█
");
pub const ICON_CROSS: IconRaw = ImageRaw::<BinaryColor>::new(&CROSS_BYTES, 5);

const OHM_BYTES: [u8; 20] = icon_bytes!("
░░░███░░░
░░█░░░█░░
░█░░░░░█░
█░░░░░░░█
█░░░░░░░█
█░░░░░░░█
░█░░░░░█░
░░█░░░█░░
░░█░░░█░░
███░░░███
");
pub const ICON_OHM: IconRaw = ImageRaw::<BinaryColor>::new(&OHM_BYTES, 9);

const CHARGING_BYTES: [u8; 11] = icon_bytes!("
░░░█░░
░░░█░░
░░██░░
░░██░█
░█████
░████░
█████░
█░██░░
░░██░░
░░█░░░
░░█░░░
");
pub const ICON_CHARGING: IconRaw = ImageRaw::<BinaryColor>::new(&CHARGING_BYTES, 6);

pub const ICON_CHARGED: IconRaw = ImageRaw::<BinaryColor>::new(&inverse(&CHARGING_BYTES), 6);

const GEAR_BYTES: [u8; 24] = icon_bytes!("
░░░░░██░░░░░
░██░░██░░██░
████████████
░██████████░
░░███░░███░░
████░░░░████
████░░░░████
░░███░░███░░
░██████████░
████████████
░██░░██░░██░
░░░░░██░░░░░
");
const SETTINGS_TAB_BYTES: [u8; 24] = icon_bytes!("
░░░░██░░░░
░░░████░░░
░████████░
██████████
████░░████
░██░░░░██░
░██░░░░██░
████░░████
██████████
░████████░
░░░████░░░
░░░░██░░░░
");
pub const ICON_TAB_SETTINGS: IconRaw = ImageRaw::<BinaryColor>::new(&SETTINGS_TAB_BYTES, 10);

pub const ICON_TAB_SETTINGS_SELECTED: IconRaw = ImageRaw::<BinaryColor>::new(&inverse(&SETTINGS_TAB_BYTES), 10);

const PUFF_TAB_BYTES: [u8; 24] = icon_bytes!("
░░█░█░░░░
░█░█░░░░░
█░░█░░░░░
█░░░█████
░█░░░░░░░
░░███████
░░███████
░█░░░░░░░
█░░░█████
█░░█░░░░░
░█░█░░░░░
░░█░█░░░░
");
pub const ICON_TAB_PUFFS: IconRaw = ImageRaw::<BinaryColor>::new(&PUFF_TAB_BYTES, 9);

pub const ICON_TAB_PUFFS_SELECTED: IconRaw = ImageRaw::<BinaryColor>::new(&inverse(&PUFF_TAB_BYTES), 9);

const BATTERY_TAB_BYTES: [u8; 24] = icon_bytes!("
░░░░█░░░░
░░░░█░░░░
░░░██░░░█
░░░██░░██
░░██████░
░░██████░
░██████░░
░██████░░
██░░██░░░
█░░░██░░░
░░░░█░░░░
░░░░█░░░░
");
pub const ICON_TAB_BATTERY: IconRaw = ImageRaw::<BinaryColor>::new(&BATTERY_TAB_BYTES, 9);

pub const ICON_TAB_BATTERY_SELECTED: IconRaw = ImageRaw::<BinaryColor>::new(&inverse(&BATTERY_TAB_BYTES), 9);

const COIL_BYTES: [u8; 10] = icon_bytes!("
░░███░░
░█████░
░░███░░
░█████░
░██░██░
░██░██░
███████
░█░█░█░
░█████░
███████
");
pub const ICON_COIL: IconRaw = ImageRaw::<BinaryColor>::new(&COIL_BYTES, 7);

const PUFF_BYTES: [u8; 20] = icon_bytes!("
░░█░░█░░░
░█░░█░░░░
█░░░░████
░█░░░░░░░
░░███████
░░███████
░█░░░░░░░
█░░░░████
░█░░█░░░░
░░█░░█░░░
");
pub const ICON_PUFF: IconRaw = ImageRaw::<BinaryColor>::new(&PUFF_BYTES, 9);

const SUM_BYTES: [u8; 20] = icon_bytes!("
████████░
░█░░░░░░█
░░█░░░░░░
░░░█░░░░░
░░░░█░░░░
░░░░█░░░░
░░░█░░░░░
░░█░░░░░░
░█░░░░░░█
████████░
");
pub const ICON_SUM: IconRaw = ImageRaw::<BinaryColor>::new(&SUM_BYTES, 9);

const BACKSPACE_BYTES: [u8; 18] = icon_bytes!("
░░░░░░░░░░░
░░░███████░
░░██░███░██
░████░█░███
██████░████
░████░█░███
░░██░███░██
░░░███████░
░░░░░░░░░░░
");
pub const ICON_BACKSPACE: IconRaw = ImageRaw::<BinaryColor>::new(&BACKSPACE_BYTES, 11);

const VOLTAGE_BYTES: [u8; 10] = icon_bytes!("
░░░█░░░
░░░█░░░
░░██░░█
░░██░██
░█████░
░█████░
██░██░░
█░░██░░
░░░█░░░
░░░█░░░
");
pub const ICON_VOLTAGE: IconRaw = ImageRaw::<BinaryColor>::new(&VOLTAGE_BYTES, 7);

const REVERSE_BYTES: [u8; 18] = icon_bytes!("
░░░█████░
░░░██████
░░░░░░███
░░█░░░░██
░██░░░███
█████████
████████░
░██░░░░░░
░░█░░░░░░
");
pub const ICON_REVERSE: IconRaw = ImageRaw::<BinaryColor>::new(&REVERSE_BYTES, 9);

const WARNING_BYTES: [u8; 9] = icon_bytes!("
██
██
██
██
██
██
░░
██
██
");
pub const ICON_WARNING: IconRaw = ImageRaw::<BinaryColor>::new(&WARNING_BYTES, 2);

const ONE_BYTES: [u8; 9] = icon_bytes!("
░██░
░██░
░██░
░██░
░██░
░██░
░██░
░██░
░██░
");
pub const ICON_ONE: IconRaw = ImageRaw::<BinaryColor>::new(&ONE_BYTES, 4);

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
pub const SUN_STROKE: IconRaw = ImageRaw::<BinaryColor>::new(&SUN_STROKE_BYTES, BRIGHTNESS_ICON_SIZE);

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
pub const SUN_FILL: IconRaw = ImageRaw::<BinaryColor>::new(&SUN_FILL_BYTES, BRIGHTNESS_ICON_SIZE);

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
pub const MOON_STROKE: IconRaw = ImageRaw::<BinaryColor>::new(&MOON_STROKE_BYTES, BRIGHTNESS_ICON_SIZE);

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
pub const MOON_FILL: IconRaw = ImageRaw::<BinaryColor>::new(&MOON_FILL_BYTES, BRIGHTNESS_ICON_SIZE);

const fn inverse<const N: usize>(arr: &[u8; N]) -> [u8; N] {
    let mut result = [0u8; N];
    let mut i = 0;
    while i < N {
        result[i] = !arr[i];
        i += 1;
    }
    return result
}
