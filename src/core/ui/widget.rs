use crate::core::charge_status::ChargeStatus;
use crate::core::graphics::{space, BATTERY_CELL, BATTERY_TEXT, BLACK_TEXT, CATHODE, TEXT_ICON_PADDING, WHITE_FILL, WHITE_STROKE, WHITE_TEXT, ZERO_POINT};
use crate::core::icons::{BRIGHTNESS_ICON_SIZE, ICON_BACKSPACE, ICON_COIL, ICON_PUFF, ICON_SUM, MOON_FILL, MOON_STROKE, SUN_FILL, SUN_STROKE};
use crate::core::icons::{ICON_CHARGED, ICON_CHARGING, ICON_CROSS, ICON_EMPTY, ICON_OHM, ICON_ONE, ICON_REVERSE, ICON_WARNING};
use crate::core::strings::{FULL, HARD, IDLE, LOAD, MEDIUM, RARE, WELL};
use crate::core::ui::place::Place;
use crate::core::ui::placing::Placing;
use crate::core::ui::power_and_limit;
use crate::data::battery::Battery;
use crate::data::power::Power;
use crate::ext::image_ext::IconRawExt;
use crate::ext::result_ext::ResultExt;
use crate::ext::str_ext::string;
use crate::ext::text_ext::TextExt;
use crate::format;
use crate::types::{Brightness, DeciOhm, DeciSecond, Display, IconRaw, MilliVolt, MilliWatt, Percent, PuffCount, Second};
use crate::values::{SCREEN_HEIGHT, SCREEN_WIDTH, VOLT, VOLTS_FULL, VOLTS_MIN, WATT};
use embedded_graphics::geometry::{Dimensions, Point, Size};
use embedded_graphics::image::Image;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{CornerRadii, Rectangle, RoundedRectangle};
use embedded_graphics::text::Text;
use embedded_layout::layout::linear::{FixedMargin, LinearLayout};
use embedded_layout::prelude::{horizontal, vertical, Align, Chain};
use heapless::String;

#[derive(Clone, PartialEq)]
pub enum Widget {
    None,
    PowerAndLimit {
        power: Power,
        limit: Second,
        edit: power_and_limit::Edit,
    },
    ResistanceAndWatt {
        resistance: DeciOhm,
        mw: Option<MilliWatt>,
        power: Power,
        edit: bool,
    },
    Statusbar {
        total: DeciSecond,
        battery: Battery,
    },
    Brightness {
        brightness: Brightness,
        focused: bool,
    },
    PuffCoil {
        duration: DeciSecond,
        reset: bool,
    },
    PuffCount {
        count: PuffCount,
        reset: bool,
    },
    PuffTotal {
        duration: DeciSecond,
        reset: bool,
    },
    BatteryIdle(Option<MilliVolt>),
    BatteryLoad(Option<MilliVolt>),
    BatteryFull(Option<MilliVolt>),
}

impl Widget {

    pub fn render(&self, display: &mut Display) {
        match self.clone() {
            Widget::None => (),
            Widget::PowerAndLimit { power, limit, edit } => Self::draw_power_and_limit(power, limit, edit, display),
            Widget::ResistanceAndWatt { resistance, mw, power, edit } => Self::draw_resistance_and_watt(resistance, mw, power, edit, display),
            Widget::Statusbar { total, battery } => Self::draw_statusbar(total, battery, display),
            Widget::Brightness { brightness, focused } => Self::draw_brightness(brightness, focused, display),
            Widget::PuffCoil { duration, reset } => Self::draw_resettable(&ICON_COIL, time(duration).as_str(), reset, Place::Top, display),
            Widget::PuffCount { count, reset } => Self::draw_resettable(&ICON_PUFF, format!(8, "{count}").as_str(), reset, Place::Middle, display),
            Widget::PuffTotal { duration, reset } => Self::draw_resettable(&ICON_SUM, time(duration).as_str(), reset, Place::Bottom, display),
            Widget::BatteryIdle(mv) => Self::draw_voltage(IDLE, mv, Place::Top, display),
            Widget::BatteryLoad(mv) => Self::draw_voltage(LOAD, mv, Place::Middle, display),
            Widget::BatteryFull(mv) => Self::draw_voltage(FULL, mv, Place::Bottom, display),
        }
    }

    fn draw_power_and_limit(
        power: Power,
        limit: Second,
        edit: power_and_limit::Edit,
        display: &mut Display,
    ) {
        let power = match power {
            Power::Rare => RARE,
            Power::Medium => MEDIUM,
            Power::Well => WELL,
            Power::Hard => HARD,
        };
        let power = Text::new(power, Point::new(0, 14), if edit.is_power() { BLACK_TEXT } else { WHITE_TEXT });
        let seconds = format!(2, "{}s", limit);
        let limit = Text::new(seconds.as_str(), Point::new(0, 14), if edit.is_limit() { BLACK_TEXT } else { WHITE_TEXT });

        let chain = Chain::new(Chain::new(power).append(power.background(edit.is_power())))
            .append(ICON_CROSS.to_icon())
            .append(Chain::new(limit).append(limit.background(edit.is_limit())));

        LinearLayout::horizontal(chain)
            .with_spacing(FixedMargin(4))
            .with_alignment(vertical::Center)
            .arrange()
            .place_center(Place::Top, display)
            .draw(display)
            .ignore();
    }

    fn draw_resistance_and_watt(
        resistance: DeciOhm,
        mw: Option<MilliWatt>,
        power: Power,
        edit: bool,
        display: &mut Display,
    ) {
        let resistance = format!(4, "{}.{}", resistance / 10, resistance % 10);
        let resistance = Text::new(resistance.as_str(), Point::new(0, 14), if edit { BLACK_TEXT } else { WHITE_TEXT });
        let watt = match watts(mw, power) {
            Some(watt) => format!(4, "{}W", watt),
            None => string::<4>("--W"),
        };
        let watt = Text::new(watt.as_str(), Point::new(0, 14), WHITE_TEXT);
        let chain = Chain::new(Chain::new(resistance).append(resistance.background(edit)))
            .append(space(2))
            .append(ICON_OHM.to_icon())
            .append(space(12))
            .append(watt)
            .append(space(3)); // because of resistance has a background
        LinearLayout::horizontal(chain)
            .with_alignment(vertical::Center)
            .arrange()
            .place_center(Place::Middle, display)
            .draw(display)
            .ignore();
    }

    fn draw_brightness(brightness: Brightness, focused: bool, display: &mut Display) {
        let icon_y = (SCREEN_HEIGHT - BRIGHTNESS_ICON_SIZE) as i32;
        let icon_x = 2;
        let moon = if focused { MOON_FILL } else { MOON_STROKE };
        Image::new(&moon, Point::new(icon_x, icon_y))
            .draw(display).ignore();
        let sun = if focused { SUN_FILL } else { SUN_STROKE };
        let icon_x = (SCREEN_WIDTH - BRIGHTNESS_ICON_SIZE) as i32 - icon_x;
        Image::new(&sun, Point::new(icon_x, icon_y))
            .draw(display).ignore();

        let dashes = 5;
        let width = 9;
        let step = width * 2;
        let offset = (SCREEN_WIDTH as i32 - width * (dashes * 2 - 1)) / 2;
        let y = (SCREEN_HEIGHT - BRIGHTNESS_ICON_SIZE / 2) as i32;
        let brightness = brightness as i32;
        for i in 0..dashes {
            let x = offset + i * step;
            let on = i <= brightness;
            let width = width as u32;
            match on || focused {
                true => RoundedRectangle::new(Rectangle::new(Point::new(x, y - 3), Size::new(width, 6)), CornerRadii::new(Size::new(2, 2)))
                    .into_styled(if on && focused { WHITE_FILL } else { WHITE_STROKE })
                    .draw(display)
                    .ignore(),
                false => Rectangle::new(Point::new(x, y - 1), Size::new(width, 2))
                    .into_styled(WHITE_STROKE)
                    .draw(display)
                    .ignore(),
            }
        }
    }

    fn draw_statusbar(
        total: DeciSecond,
        battery: Battery,
        display: &mut Display,
    ) {
        let display_area = display.bounding_box();

        let mut tmp = total;
        let ds = tmp % 10;
        tmp /= 10;
        let s = tmp % 60;
        tmp /= 60;
        let m = tmp % 60;
        let h = tmp / 60;

        let time = format!(16, "{h}:{m}'{s}.{ds}\"");
        let text = Text::new(time.as_str(), ZERO_POINT, WHITE_TEXT);
        text.align_to(&display_area, horizontal::Left, vertical::Bottom)
            .draw(display)
            .ignore();

        let full = battery.full.unwrap_or(VOLTS_FULL);
        let level = battery.idle.and_then(|idle| {
            let percents = (idle - VOLTS_MIN) as u32 * 100 / (full - VOLTS_MIN) as u32;
            Some(percents.clamp(0, 100) as Percent)
        });
        let icon = match battery.status {
            ChargeStatus::Discharging => match &level {
                Some(100) => ICON_ONE.to_icon(),
                _ => ICON_EMPTY.to_icon(),
            },
            ChargeStatus::Charging => ICON_CHARGING.to_icon(),
            ChargeStatus::Full => ICON_CHARGED.to_icon(),
            ChargeStatus::Reverse =>  ICON_REVERSE.to_icon(),
            ChargeStatus::Unknown => ICON_WARNING.to_icon(),
        };
        let amount = match battery.status {
            ChargeStatus::Discharging => match level {
                None => string::<3>("?"),
                Some(100) => string::<3>("00"),
                Some(value) => format!(3, "{value}"),
            },
            _ => string::<3>(""),
        };
        let text = Text::new(amount.as_str(), ZERO_POINT, BATTERY_TEXT);
        let battery_style = match battery.status {
            ChargeStatus::Full => WHITE_FILL,
            _ => WHITE_STROKE,
        };
        let content = LinearLayout::horizontal(Chain::new(icon).append(text))
            .with_alignment(vertical::Center)
            .arrange();
        let battery_cell = BATTERY_CELL.into_styled(battery_style);
        let content = content.align_to(&battery_cell, horizontal::Center, vertical::Center);
        let battery_cell = Chain::new(content).append(battery_cell);
        let battery = Chain::new(CATHODE.into_styled(WHITE_STROKE)).append(battery_cell);
        LinearLayout::horizontal(battery)
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Right, vertical::Bottom)
            .draw(display)
            .ignore();
    }

    fn draw_resettable(
        icon: &IconRaw,
        text: &str,
        reset: bool,
        place: Place,
        display: &mut Display,
    ) {
        let chain = Chain::new(icon.to_icon())
            .append(space(TEXT_ICON_PADDING))
            .append(Text::new(text, ZERO_POINT, WHITE_TEXT))
            .append(space(if reset { TEXT_ICON_PADDING } else { 0 }))
            .append(if reset { ICON_BACKSPACE.to_icon() } else { ICON_EMPTY.to_icon() });

        LinearLayout::horizontal(chain)
            .with_alignment(vertical::Center)
            .arrange()
            .place_center(place, display)
            .draw(display)
            .ignore();
    }

    fn draw_voltage(
        label: &str,
        mv: Option<MilliVolt>,
        place: Place,
        display: &mut Display,
    ) {
        let volts = mv
            .map(|mv| format!(12, "{label} {:.3}v", mv as f32 / VOLT as f32))
            .unwrap_or_else(|| format!(12, "{label} -.---v"));

        Text::new(volts.as_str(), ZERO_POINT, WHITE_TEXT)
            .place_center(place, display)
            .draw(display)
            .ignore();
    }
}

fn time(time: DeciSecond) -> String::<16> {
    let mut tmp = time;
    let ds = tmp % 10;
    tmp /= 10;
    let s = tmp % 60;
    tmp /= 60;
    let m = tmp % 60;
    let h = tmp / 60;
    return format!(16, "{h}:{m}'{s}.{ds}\"")
}

fn watts(
    mw: Option<MilliWatt>,
    power: Power,
) -> Option<u32> {
    let mut mw = mw?;
    let percents = power.percents() as MilliWatt;
    mw = mw * percents / 100;
    let mut watts = mw / WATT;
    if (mw % WATT) >= (WATT / 2) {
        watts += 1;
    }
    return Some(watts as u32)
}

