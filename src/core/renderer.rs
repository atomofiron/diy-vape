use crate::core::cleaner::Cleaner;
use crate::core::graphics::{space, BATTERY_TEXT, BLACK_FILL, BLACK_STROKE, BLACK_TEXT, ICON_CHARGING, ICON_CROSS, ICON_OHM, WHITE_FILL, WHITE_STROKE, WHITE_TEXT};
use crate::core::graphics::{AREA, OFFSET, RADIUS, VISUAL_BASELINE_14};
use crate::core::strings::{HARD, LIMIT, MEDIUM, POWER, RARE, RESISTANCE, WELL};
use crate::data::mode::Mode;
use crate::data::power::Power;
use crate::data::state::State;
use crate::ext::result_ext::ResultExt;
use crate::ext::text_ext::TextExt;
use crate::types::Display;
use crate::values::{PROGRESS_OFFSET, PROGRESS_STEP, PROGRESS_WIDTH, SCREEN_WIDTH};
use crate::{format, kopy};
use core::cmp::min;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, CornerRadii, Line, Rectangle, RoundedRectangle};
use embedded_graphics::text::Text;
use embedded_layout::layout::linear::{FixedMargin, LinearLayout};
use embedded_layout::prelude::{horizontal, vertical, Align, Chain};
use embedded_layout::View;

pub trait Renderer {
    fn render_all(&mut self, display: &mut Display);

    fn draw_header(&mut self, display: &mut Display);
    fn render_header(&mut self, display: &mut Display);

    fn draw_power_and_limit(&mut self, display: &mut Display);
    fn render_power_and_limit(&mut self, display: &mut Display);

    fn draw_resistance_and_watt(&mut self, display: &mut Display);
    fn render_resistance_and_watt(&mut self, display: &mut Display);

    fn draw_footer(&mut self, display: &mut Display);
    fn render_footer(&mut self, display: &mut Display);

    fn draw_buttons(&mut self, display: &mut Display);
    fn render_buttons(&mut self, display: &mut Display);
}

impl Renderer for State {

    fn render_all(&mut self, display: &mut Display) {
        self.draw_header(display);
        self.draw_power_and_limit(display);
        self.draw_resistance_and_watt(display);
        self.draw_footer(display);

        display.flush().ignore();
    }

    fn draw_header(&mut self, display: &mut Display) {
        match self.mode {
            Mode::Work(progress) => {
                display.clear_header(false);
                let point = Point::new(PROGRESS_OFFSET, 0);
                let size = Size::new(PROGRESS_WIDTH, AREA);
                let corners = CornerRadii::new(Size::new(RADIUS, RADIUS));
                let progress = progress as u32 / PROGRESS_STEP;
                let fill_size = kopy!(size, width = min(progress + AREA, PROGRESS_WIDTH));
                RoundedRectangle::new(Rectangle::new(point, fill_size), corners)
                    .into_styled(WHITE_FILL)
                    .draw(display)
                    .ignore();
                let cut_point = kopy!(point, x = point.x + progress as i32);
                Rectangle::new(cut_point, Size::new(AREA, AREA))
                    .into_styled(BLACK_FILL)
                    .draw(display)
                    .ignore();
                RoundedRectangle::new(Rectangle::new(point, size), corners)
                    .into_styled(WHITE_STROKE)
                    .draw(display)
                    .ignore();
                self.draw_buttons(display);
            }
            Mode::Power => {
                display.clear_header(true);
                let title =  format!(10, "{POWER} {}%", self.config.power.value());
                Text::new(title.as_str(), Point::new(0, VISUAL_BASELINE_14), BLACK_TEXT)
                    .center()
                    .draw(display)
                    .ignore();
            }
            Mode::Limit => {
                display.clear_header(true);
                Text::new(LIMIT, Point::new(0, VISUAL_BASELINE_14), BLACK_TEXT)
                    .center()
                    .draw(display)
                    .ignore();
            }
            Mode::Resistance => {
                display.clear_header(true);
                Text::new(RESISTANCE, Point::new(0, VISUAL_BASELINE_14), BLACK_TEXT)
                    .center()
                    .draw(display)
                    .ignore();
            }
        };
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

        let resistance = format!(6, "{}", self.config.resistance as f32 / 10.0);
        let resistance = Text::new(resistance.as_str(), Point::new(0, 14), if is_resistance { BLACK_TEXT } else { WHITE_TEXT });
        let watt = format!(4, "{}W", self.watt);
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

    fn draw_footer(&mut self, display: &mut Display) {
        display.clear_footer();

        let display_area = display.bounding_box();

        let mut tmp = self.total;
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

        let charging = ICON_CHARGING.into_styled(if self.battery_charging { WHITE_STROKE } else { BLACK_STROKE });
        let cathode = Line::new(Point::new(0, 0), Point::new(0, 4))
            .into_styled(WHITE_STROKE);
        let battery = RoundedRectangle::new(Rectangle::new(Point::new(0, 3), Size::new(21, 13)), CornerRadii::new(Size::new(4, 4)))
            .into_styled(WHITE_STROKE);
        let (amount, x) = match self.battery_level {
            None => (format!(3, "?"), 8),
            Some(100) => (format!(3, "00"), 6),
            Some(value) => (format!(3, "{value}"), 4),
        };
        let one = if let Some(100) = self.battery_level { "|" } else { "" };
        let one = Text::new(one, Point::new(1, 13), BATTERY_TEXT);
        let percents = Text::new(amount.as_str(), Point::new(x, 13), BATTERY_TEXT);

        let chain = Chain::new(charging)
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

    fn render_footer(&mut self, display: &mut Display) {
        self.draw_footer(display);
        display.flush().ignore();
    }

    fn draw_buttons(&mut self, display: &mut Display) {
        let y = match self.mode {
            Mode::Work(_) => 7,
            Mode::Power |
            Mode::Limit => 23 + OFFSET,
            Mode::Resistance => 39 + OFFSET,
        };
        Circle::with_center(Point::new(RADIUS as i32 - 1, y), AREA)
            .into_styled(if self.buttons.0 { WHITE_FILL } else { WHITE_STROKE })
            .draw(display)
            .ignore();
        Circle::with_center(Point::new((SCREEN_WIDTH - RADIUS) as i32 - 1, y), AREA)
            .into_styled(if self.buttons.1 { WHITE_FILL } else { WHITE_STROKE })
            .draw(display)
            .ignore();
    }

    fn render_buttons(&mut self, display: &mut Display) {
        self.draw_buttons(display);
        display.flush().ignore();
    }
}
