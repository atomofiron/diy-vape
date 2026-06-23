use crate::core::charge_status::ChargeStatus;
use crate::core::cleaner::Cleaner;
use crate::core::graphics::{space, BATTERY_CELL, BATTERY_TEXT, BLACK_BOLD_TEXT, BLACK_FILL, BLACK_TEXT, CATHODE, CORNER_RADII, FIRST_TAB_RECTANGLE, HEADER_POINT, HEADER_RECTANGLE, HEADER_SIZE, SECOND_TAB_RECTANGLE, TAB_MARGIN, TAB_PUFFS, TAB_SETTINGS, TEXT_ICON_MARGIN, WHITE_FILL, WHITE_STROKE, WHITE_TEXT};
use crate::core::graphics::{AREA, OFFSET, RADIUS, VISUAL_BASELINE_14};
use crate::core::icons::{BRIGHTNESS_ICON_SIZE, ICON_PUFF, ICON_VOLTAGE, MOON_FILL, MOON_STROKE, SUN_FILL, SUN_STROKE};
use crate::core::icons::{ICON_CHARGED, ICON_CHARGING, ICON_CROSS, ICON_EMPTY, ICON_OHM, ICON_ONE, ICON_REVERSE, ICON_WARNING};
use crate::core::strings::{BRIGHTNESS, HARD, LIMIT, MEDIUM, POWER, RARE, RESET_ALL, RESET_COIL, RESISTANCE, WELL};
use crate::data::mode::Mode;
use crate::data::power::Power;
use crate::data::state::State;
use crate::data::tab::Tab;
use crate::ext::image_ext::IconRawExt;
use crate::ext::result_ext::ResultExt;
use crate::ext::str_ext::string;
use crate::ext::text_ext::TextExt;
use crate::types::{Display, Progress, Time};
use crate::values::{HEADER_WIDTH, PROGRESS_MAX, PROGRESS_STEP, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{format, kopy};
use core::cmp::min;
use embedded_graphics::image::Image;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, CornerRadii, Rectangle, RoundedRectangle};
use embedded_graphics::text::Text;
use embedded_layout::layout::linear::{FixedMargin, LinearLayout};
use embedded_layout::prelude::{horizontal, vertical, Align, Chain};
use embedded_layout::View;

pub trait Renderer {
    fn render_dirty(&mut self, display: &mut Display);
}

trait RendererImpl {
    fn draw_header(&mut self, display: &mut Display);
    fn draw_tabs(selected: &Tab, display: &mut Display);
    fn draw_power_and_limit(&mut self, display: &mut Display);
    fn draw_resistance_and_watt(&mut self, display: &mut Display);
    fn draw_status(&mut self, display: &mut Display);
    fn draw_brightness(&mut self, display: &mut Display);
    fn draw_statusbar(&mut self, display: &mut Display);
    fn draw_buttons(&mut self, display: &mut Display);
    fn draw_dirty(&mut self, display: &mut Display);
}

impl Renderer for State {

    fn render_dirty(&mut self, display: &mut Display) {
        if self.is_smth_dirty() {
            self.draw_dirty(display);
            display.flush().ignore();
        }
    }
}

impl RendererImpl for State {

    fn draw_header(&mut self, display: &mut Display) {
        let title = match &self.mode {
            Mode::Work { duration, .. } => {
                display.clear_header(false);
                let point = HEADER_POINT;
                let size = HEADER_SIZE;
                let limit = self.limit_ms();
                let max = PROGRESS_MAX as Time;
                let progress = min(duration * max / limit, max) as Progress;
                let progress = (progress / PROGRESS_STEP) as u32;
                let fill_size = kopy!(size, width = min(progress + AREA, HEADER_WIDTH));
                RoundedRectangle::new(Rectangle::new(point, fill_size), CORNER_RADII)
                    .into_styled(WHITE_FILL)
                    .draw(display)
                    .ignore();
                let cut_point = kopy!(point, x = point.x + progress as i32);
                let cut_width = min(AREA, HEADER_WIDTH - progress);
                Rectangle::new(cut_point, Size::new(cut_width, AREA))
                    .into_styled(BLACK_FILL)
                    .draw(display)
                    .ignore();
                HEADER_RECTANGLE
                    .into_styled(WHITE_STROKE)
                    .draw(display)
                    .ignore();
                return
            },
            Mode::Tabs(selected) => return Self::draw_tabs(&selected, display),
            Mode::Power => {
                display.clear_header(true);
                let title = format!(10, "{POWER} {}%", self.config.power.percents());
                Text::new(title.as_str(), Point::new(0, VISUAL_BASELINE_14), BLACK_BOLD_TEXT)
                    .center()
                    .draw(display)
                    .ignore();
                return
            },
            Mode::Limit => LIMIT,
            Mode::Resistance => RESISTANCE,
            Mode::Brightness => BRIGHTNESS,
            Mode::ResetCoil => RESET_COIL,
            Mode::ResetStats => RESET_ALL,
        };
        display.clear_header(true);
        Text::new(title, Point::new(0, VISUAL_BASELINE_14), BLACK_BOLD_TEXT)
            .center()
            .draw(display)
            .ignore();
    }

    fn draw_tabs(selected: &Tab, display: &mut Display) {
        display.clear_header(false);
        let display_area = display.bounding_box();
        
        let view = TAB_SETTINGS.peek(*selected == Tab::Settings);
        let tab = FIRST_TAB_RECTANGLE.into_styled(view.background);
        let icon = view.icon.to_icon().align_to(&tab, horizontal::Center, vertical::Center);
        let first = Chain::new(icon).append(tab);

        let view = TAB_PUFFS.peek(*selected == Tab::Puffs);
        let tab = SECOND_TAB_RECTANGLE.into_styled(view.background);
        let icon = view.icon.to_icon().align_to(&tab, horizontal::Center, vertical::Center);
        let second = Chain::new(icon).append(tab);

        LinearLayout::horizontal(Chain::new(first).append(second))
            .with_spacing(TAB_MARGIN)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Top)
            .draw(display)
            .ignore();
    }

    fn draw_power_and_limit(&mut self, display: &mut Display) {
        display.clear_top_body();

        let display_area = display.bounding_box();
        let is_power = matches!(self.mode, Mode::Power);
        let is_limit = matches!(self.mode, Mode::Limit);

        let power = match self.config.power {
            Power::Rare => RARE,
            Power::Medium => MEDIUM,
            Power::Well => WELL,
            Power::Hard => HARD,
        };
        let power = Text::new(power, Point::new(0, 14), if is_power { BLACK_TEXT } else { WHITE_TEXT });
        let seconds = format!(2, "{}s", self.config.limit);
        let limit = Text::new(seconds.as_str(), Point::new(0, 14), if is_limit { BLACK_TEXT } else { WHITE_TEXT });

        let chain = Chain::new(Chain::new(power).append(power.background(is_power)))
            .append(ICON_CROSS.to_icon())
            .append(Chain::new(limit).append(limit.background(is_limit)));

        LinearLayout::horizontal(chain)
            .with_spacing(FixedMargin(4))
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .translate(Point::new(0, -8 + OFFSET))
            .draw(display)
            .ignore();
    }

    fn draw_resistance_and_watt(&mut self, display: &mut Display) {
        display.clear_bottom_body();

        let display_area = display.bounding_box();
        let is_resistance = matches!(self.mode, Mode::Resistance);

        let resistance = format!(4, "{}.{}", self.config.resistance / 10, self.config.resistance % 10);
        let resistance = Text::new(resistance.as_str(), Point::new(0, 14), if is_resistance { BLACK_TEXT } else { WHITE_TEXT });
        let watt = match self.watts() {
            Some(watt) => format!(4, "{}W", watt),
            None => string::<4>("--W"),
        };
        let watt = Text::new(watt.as_str(), Point::new(0, 14), WHITE_TEXT);
        let chain = Chain::new(Chain::new(resistance).append(resistance.background(is_resistance)))
            .append(space(2))
            .append(ICON_OHM.to_icon())
            .append(space(12))
            .append(watt)
            .append(space(3)); // because of resistance has a background
        LinearLayout::horizontal(chain)
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .translate(Point::new(0, 8 + OFFSET))
            .draw(display)
            .ignore();
    }

    fn draw_status(&mut self, display: &mut Display) {
        display.clear_footer();

        let display_area = display.bounding_box();
        let count = format!(10, "{}", self.stats.count);
        let counter = Chain::new(ICON_PUFF.to_icon())
            .append(Text::new(count.as_str(), Point::new(0, 0), WHITE_TEXT));

        LinearLayout::horizontal(counter)
            .with_spacing(TEXT_ICON_MARGIN)
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Left, vertical::Bottom)
            .draw(display)
            .ignore();

        let volts = self.battery.idle
            .map(|mv| format!(6, "{:.3}v", mv as f32 / 1000.0))
            .unwrap_or_else(|| string::<6>("-.---v"));
        let voltage = Chain::new(ICON_VOLTAGE.to_icon())
            .append(Text::new(volts.as_str(), Point::new(0, 0), WHITE_TEXT));

        LinearLayout::horizontal(voltage)
            .with_spacing(TEXT_ICON_MARGIN)
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Right, vertical::Bottom)
            .draw(display)
            .ignore();
    }

    fn draw_brightness(&mut self, display: &mut Display) {
        display.clear_footer();

        let icon_y = (SCREEN_HEIGHT - BRIGHTNESS_ICON_SIZE) as i32;
        let icon_x = 2;
        let moon = if self.mode.is_brightness() { MOON_FILL } else { MOON_STROKE };
        Image::new(&moon, Point::new(icon_x, icon_y))
            .draw(display).ignore();
        let sun = if self.mode.is_brightness() { SUN_FILL } else { SUN_STROKE };
        let icon_x = (SCREEN_WIDTH - BRIGHTNESS_ICON_SIZE) as i32 - icon_x;
        Image::new(&sun, Point::new(icon_x, icon_y))
            .draw(display).ignore();

        let dashes = 5;
        let width = 9;
        let step = width * 2;
        let offset = (SCREEN_WIDTH as i32 - width * (dashes * 2 - 1)) / 2;
        let y = (SCREEN_HEIGHT - BRIGHTNESS_ICON_SIZE / 2) as i32;
        let brightness = self.config.brightness as i32;
        let focused = self.mode.is_brightness();
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

    fn draw_statusbar(&mut self, display: &mut Display) {
        display.clear_footer();

        let display_area = display.bounding_box();

        let mut tmp = self.calc_stat_total();
        let ds = tmp % 10;
        tmp /= 10;
        let s = tmp % 60;
        tmp /= 60;
        let m = tmp % 60;
        let h = tmp / 60;

        let time = format!(16, "{h}:{m}'{s}.{ds}\"");
        Text::new(time.as_str(), Point::new(0, 62), WHITE_TEXT)
            .draw(display)
            .ignore();

        let level = self.get_battery_level();
        let icon = match self.battery.status {
            ChargeStatus::Discharging => match &level {
                Some(100) => ICON_ONE.to_icon(),
                _ => ICON_EMPTY.to_icon(),
            },
            ChargeStatus::Charging => ICON_CHARGING.to_icon(),
            ChargeStatus::Full => ICON_CHARGED.to_icon(),
            ChargeStatus::Reverse =>  ICON_REVERSE.to_icon(),
            ChargeStatus::Unknown => ICON_WARNING.to_icon(),
        };
        let amount = match self.battery.status {
            ChargeStatus::Discharging => match level {
                None => string::<3>("?"),
                Some(100) => string::<3>("00"),
                Some(value) => format!(3, "{value}"),
            },
            _ => string::<3>(""),
        };
        let text = Text::new(amount.as_str(), Point::new(0, 0), BATTERY_TEXT);
        let battery_style = match self.battery.status {
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

    fn draw_buttons(&mut self, display: &mut Display) {
        let delta = RADIUS as i32 - 1;
        let left_center = Point::new(delta, delta);
        let right_center = Point::new((SCREEN_WIDTH - RADIUS) as i32 - 1, delta);
        if !self.buttons.left {
            Circle::with_center(left_center, AREA)
                .into_styled(BLACK_FILL)
                .draw(display)
                .ignore();
        }
        if !self.buttons.right {
            Circle::with_center(right_center, AREA)
                .into_styled(BLACK_FILL)
                .draw(display)
                .ignore();
        }
        Circle::with_center(left_center, AREA)
            .into_styled(if self.buttons.left { WHITE_FILL } else { WHITE_STROKE })
            .draw(display)
            .ignore();
        Circle::with_center(right_center, AREA)
            .into_styled(if self.buttons.right { WHITE_FILL } else { WHITE_STROKE })
            .draw(display)
            .ignore();
    }

    fn draw_dirty(&mut self, display: &mut Display) {
        if self.is_header_dirty {
            self.draw_header(display);
        }
        if self.are_buttons_dirty {
            self.draw_buttons(display);
        }
        if self.is_power_or_limit_dirty {
            self.draw_power_and_limit(display);
        }
        if self.is_resistance_or_watts_dirty {
            self.draw_resistance_and_watt(display);
        }
        if self.is_statusbar_dirty && self.mode.is_work() {
            self.draw_statusbar(display);
        }
        if self.is_status_dirty && self.mode.is_puffs() {
            self.draw_status(display);
        }
        if self.is_brightness_dirty && self.mode.is_settings() {
            self.draw_brightness(display);
        }
        self.are_buttons_dirty = false;
        self.is_header_dirty = false;
        self.is_power_or_limit_dirty = false;
        self.is_resistance_or_watts_dirty = false;
        self.is_statusbar_dirty = false;
        self.is_status_dirty = false;
        self.is_brightness_dirty = false;
    }
}
