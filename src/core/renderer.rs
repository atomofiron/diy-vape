use crate::core::charge_status::ChargeStatus;
use crate::core::cleaner::Cleaner;
use crate::core::graphics::{space, BATTERY_TEXT, BLACK_BOLD_TEXT, BLACK_FILL, BLACK_TEXT, CORNER_RADII, HEADER_POINT, HEADER_RECTANGLE, HEADER_SIZE, ICON_CHARGED, ICON_CHARGING, ICON_CROSS, ICON_EMPTY, ICON_OHM, ICON_WARNING, WHITE_FILL, WHITE_STROKE, WHITE_TEXT};
use crate::core::graphics::{AREA, OFFSET, RADIUS, VISUAL_BASELINE_14};
use crate::core::strings::{BRIGHTNESS, HARD, LIMIT, MEDIUM, POWER, RARE, RESISTANCE, WELL};
use crate::data::mode::Mode;
use crate::data::power::Power;
use crate::data::state::State;
use crate::ext::result_ext::ResultExt;
use crate::ext::str_ext::string;
use crate::ext::text_ext::TextExt;
use crate::types::{Display, Progress, Time};
use crate::values::{HEADER_OFFSET, HEADER_WIDTH, PROGRESS_MAX, PROGRESS_STEP, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{format, kopy};
use core::cmp::min;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, CornerRadii, Line, Rectangle, RoundedRectangle};
use embedded_graphics::text::Text;
use embedded_layout::layout::linear::{FixedMargin, LinearLayout};
use embedded_layout::prelude::{horizontal, vertical, Align, Chain};
use embedded_layout::View;

pub trait Renderer {

    fn draw_header(&mut self, display: &mut Display);
    fn render_header(&mut self, display: &mut Display);

    fn draw_power_and_limit(&mut self, display: &mut Display);
    fn render_power_and_limit(&mut self, display: &mut Display);

    fn draw_resistance_and_watt(&mut self, display: &mut Display);
    fn render_resistance_and_watt(&mut self, display: &mut Display);

    fn draw_brightness(&mut self, display: &mut Display);
    fn render_brightness(&mut self, display: &mut Display);

    fn draw_statusbar(&mut self, display: &mut Display);
    fn render_statusbar(&mut self, display: &mut Display);

    fn draw_buttons(&mut self, display: &mut Display);
    fn render_buttons(&mut self, display: &mut Display);

    fn draw_dirty(&mut self, display: &mut Display);
    fn render_dirty(&mut self, display: &mut Display);
}

impl Renderer for State {

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
                Rectangle::new(cut_point, Size::new(AREA, AREA))
                    .into_styled(BLACK_FILL)
                    .draw(display)
                    .ignore();
                HEADER_RECTANGLE
                    .into_styled(WHITE_STROKE)
                    .draw(display)
                    .ignore();
                self.draw_buttons(display);
                return
            }
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
        };
        display.clear_header(true);
        Text::new(title, Point::new(0, VISUAL_BASELINE_14), BLACK_BOLD_TEXT)
            .center()
            .draw(display)
            .ignore();
    }

    fn render_header(&mut self, display: &mut Display) {
        self.draw_header(display);
        display.flush().ignore();
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
            .append(ICON_CROSS)
            .append(Chain::new(limit).append(limit.background(is_limit)));

        LinearLayout::horizontal(chain)
            .with_spacing(FixedMargin(4))
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Center)
            .translate(Point::new(0, -8 + OFFSET))
            .draw(display)
            .ignore();
        if is_power || is_limit {
            self.draw_buttons(display);
        }
    }

    fn render_power_and_limit(&mut self, display: &mut Display) {
        self.draw_power_and_limit(display);
        display.flush().ignore();
    }

    fn draw_resistance_and_watt(&mut self, display: &mut Display) {
        display.clear_bottom_body();

        let display_area = display.bounding_box();
        let is_resistance = matches!(self.mode, Mode::Resistance);

        let resistance = format!(6, "{:.1}", self.config.resistance as f32 / 10.0);
        let resistance = Text::new(resistance.as_str(), Point::new(0, 14), if is_resistance { BLACK_TEXT } else { WHITE_TEXT });
        let watt = match self.watts() {
            Some(watt) => format!(4, "{}W", watt),
            None => string::<4>("--W"),
        };
        let watt = Text::new(watt.as_str(), Point::new(0, 14), WHITE_TEXT);
        let chain = Chain::new(Chain::new(resistance).append(resistance.background(is_resistance)))
            .append(space(2))
            .append(ICON_OHM)
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
        if is_resistance {
            self.draw_buttons(display);
        }
    }

    fn render_resistance_and_watt(&mut self, display: &mut Display) {
        self.draw_resistance_and_watt(display);
        display.flush().ignore();
    }

    fn draw_brightness(&mut self, display: &mut Display) {
        display.clear_footer();
        let step = HEADER_WIDTH / 9 * 2;
        let offset = HEADER_OFFSET + (HEADER_WIDTH - step * 9 / 2) as i32 / 2;
        let y = (SCREEN_HEIGHT - AREA / 2) as i32;
        let brightness = self.config.brightness as i32;
        let focused = self.mode == Mode::Brightness;
        for i in 0..5 {
            let x = offset + i * step as i32;
            let on = i <= brightness;
            match on || focused {
                true => RoundedRectangle::new(Rectangle::new(Point::new(x, y), Size::new(step / 2, 4)), CornerRadii::new(Size::new(2, 2)))
                    .into_styled(if on && focused { WHITE_FILL } else { WHITE_STROKE })
                    .draw(display)
                    .ignore(),
                false => Rectangle::new(Point::new(x + 1, y + 1), Size::new(step / 2 - 2, 2))
                    .into_styled(WHITE_STROKE)
                    .draw(display)
                    .ignore(),
            }
        }
    }

    fn render_brightness(&mut self, display: &mut Display) {
        self.draw_brightness(display);
        display.flush().ignore();
    }

    fn draw_statusbar(&mut self, display: &mut Display) {
        display.clear_footer();

        let display_area = display.bounding_box();

        let mut tmp = self.stats.total;
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

        let status = match self.battery.status {
            ChargeStatus::Discharging => ICON_EMPTY,
            ChargeStatus::Charging => ICON_CHARGING,
            ChargeStatus::Full => ICON_CHARGED,
            ChargeStatus::Unknown => ICON_WARNING,
        }.into_styled(WHITE_STROKE);
        let cathode = Line::new(Point::new(0, 0), Point::new(0, 4))
            .into_styled(WHITE_STROKE);
        let battery = RoundedRectangle::new(Rectangle::new(Point::new(0, 3), Size::new(21, 13)), CornerRadii::new(Size::new(4, 4)))
            .into_styled(WHITE_STROKE);
        let (amount, x) = match self.battery.level {
            None => (format!(3, "?"), 8),
            Some(100) => (format!(3, "00"), 6),
            Some(value) => (format!(3, "{value}"), 4),
        };
        let one = if let Some(100) = self.battery.level { "|" } else { "" };
        let one = Text::new(one, Point::new(1, 13), BATTERY_TEXT);
        let percents = Text::new(amount.as_str(), Point::new(x, 13), BATTERY_TEXT);

        let chain = Chain::new(status)
            .append(space(3))
            .append(cathode)
            .append(Chain::new(one).append(percents).append(battery));
        LinearLayout::horizontal(chain)
            .with_alignment(vertical::Center)
            .arrange()
            .align_to(&display_area, horizontal::Right, vertical::Bottom)
            .draw(display)
            .ignore();
    }

    fn render_statusbar(&mut self, display: &mut Display) {
        self.draw_statusbar(display);
        display.flush().ignore();
    }

    fn draw_buttons(&mut self, display: &mut Display) {
        let delta = RADIUS as i32 - 1;
        let left_center = Point::new(delta, delta);
        let right_center = Point::new((SCREEN_WIDTH - RADIUS) as i32 - 1, delta);
        if !self.buttons.0 {
            Circle::with_center(left_center, AREA)
                .into_styled(BLACK_FILL)
                .draw(display)
                .ignore();
        }
        if !self.buttons.1 {
            Circle::with_center(right_center, AREA)
                .into_styled(BLACK_FILL)
                .draw(display)
                .ignore();
        }
        Circle::with_center(left_center, AREA)
            .into_styled(if self.buttons.0 { WHITE_FILL } else { WHITE_STROKE })
            .draw(display)
            .ignore();
        Circle::with_center(right_center, AREA)
            .into_styled(if self.buttons.1 { WHITE_FILL } else { WHITE_STROKE })
            .draw(display)
            .ignore();
    }

    fn render_buttons(&mut self, display: &mut Display) {
        self.draw_buttons(display);
        display.flush().ignore();
    }

    fn draw_dirty(&mut self, display: &mut Display) {
        if self.is_header_dirty {
            self.draw_header(display);
            self.is_header_dirty = false;
        }
        if self.is_power_or_limit_dirty {
            self.draw_power_and_limit(display);
            self.is_power_or_limit_dirty = false;
        }
        if self.is_resistance_or_watts_dirty {
            self.draw_resistance_and_watt(display);
            self.is_resistance_or_watts_dirty = false;
        }
        if self.is_statusbar_dirty && self.is_work() {
            self.draw_statusbar(display);
            self.is_statusbar_dirty = false;
        }
        if self.is_brightness_dirty && !self.is_work() {
            self.draw_brightness(display);
            self.is_brightness_dirty = false;
        }
        if self.are_buttons_dirty {
            self.draw_buttons(display);
            self.are_buttons_dirty = false;
        }
    }

    fn render_dirty(&mut self, display: &mut Display) {
        if self.is_smth_dirty() {
            self.draw_dirty(display);
            display.flush().ignore();
        }
    }
}
