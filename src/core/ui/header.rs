use crate::core::graphics::{BLACK_BOLD_TEXT, BLACK_FILL, CORNER_RADII, FIRST_TAB_RECTANGLE, HEADER_HEIGHT, HEADER_POINT, HEADER_RECTANGLE, HEADER_SIZE, LAST_TAB_RECTANGLE, MIDDLE_TAB_RECTANGLE, TAB_BATTERY, TAB_MARGIN, TAB_PUFFS, TAB_SETTINGS, VISUAL_BASELINE_14, WHITE_FILL, WHITE_STROKE};
use crate::core::ui::tab::Tab;
use crate::ext::image_ext::IconRawExt;
use crate::ext::result_ext::ResultExt;
use crate::ext::text_ext::TextExt;
use crate::kopy;
use crate::types::{Display, Progress};
use crate::values::{HEADER_WIDTH, PUFF_PROGRESS_STEP};
use core::cmp::min;
use embedded_graphics::geometry::{Dimensions, Point, Size};
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Rectangle, RoundedRectangle};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use embedded_layout::align::{horizontal, vertical};
use embedded_layout::layout::linear::LinearLayout;
use embedded_layout::object_chain::Chain;
use embedded_layout::prelude::Align;
use embedded_layout::View;
use strum::EnumIs;

#[derive(Clone, PartialEq, EnumIs)]
pub enum Header {
    None,
    Progress(Progress),
    Title(&'static str),
    Tabs(Tab),
}

impl Header {

    pub fn render(&self, display: &mut Display) {
        match self {
            Header::None => (),
            Header::Progress(progress) => Self::draw_progress(*progress, display),
            Header::Title(title) => Self::draw_title(title, display),
            Header::Tabs(selected) => Self::draw_tabs(selected, display),
        }
    }

    fn draw_progress(progress: Progress, display: &mut Display) {
        let progress = (progress / PUFF_PROGRESS_STEP) as u32;
        let fill_size = kopy!(HEADER_SIZE, width = min(progress + HEADER_HEIGHT, HEADER_WIDTH));
        let point = HEADER_POINT;
        RoundedRectangle::new(Rectangle::new(point, fill_size), CORNER_RADII)
            .into_styled(WHITE_FILL)
            .draw(display)
            .ignore();
        let cut_point = kopy!(point, x = point.x + progress as i32);
        let cut_width = min(HEADER_HEIGHT, HEADER_WIDTH - progress);
        Rectangle::new(cut_point, Size::new(cut_width, HEADER_HEIGHT))
            .into_styled(BLACK_FILL)
            .draw(display)
            .ignore();
        HEADER_RECTANGLE
            .into_styled(WHITE_STROKE)
            .draw(display)
            .ignore();
    }

    fn draw_title(title: &str, display: &mut Display) {
        Text::new(title, Point::new(0, VISUAL_BASELINE_14), BLACK_BOLD_TEXT)
            .center()
            .draw(display)
            .ignore();
    }

    fn draw_tabs(selected: &Tab, display: &mut Display) {
        let display_area = display.bounding_box();

        let view = TAB_SETTINGS.peek(selected.is_settings());
        let tab = FIRST_TAB_RECTANGLE.into_styled(view.background);
        let icon = view.icon.to_icon()
            .align_to(&tab, horizontal::Center, vertical::Center)
            .translate(Point::new(1, 0));
        let first = Chain::new(icon).append(tab);

        let view = TAB_PUFFS.peek(selected.is_puffs());
        let tab = MIDDLE_TAB_RECTANGLE.into_styled(view.background);
        let icon = view.icon.to_icon()
            .align_to(&tab, horizontal::Center, vertical::Center);
        let middle = Chain::new(icon).append(tab);

        let view = TAB_BATTERY.peek(selected.is_battery());
        let tab = LAST_TAB_RECTANGLE.into_styled(view.background);
        let icon = view.icon.to_icon()
            .align_to(&tab, horizontal::Center, vertical::Center)
            .translate(Point::new(-1, 0));
        let last = Chain::new(icon).append(tab);

        LinearLayout::horizontal(Chain::new(first).append(middle).append(last))
            .with_spacing(TAB_MARGIN)
            .arrange()
            .align_to(&display_area, horizontal::Center, vertical::Top)
            .draw(display)
            .ignore();
    }
}
