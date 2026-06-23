use crate::core::icons::{ICON_TAB_PUFFS, ICON_TAB_PUFFS_SELECTED, ICON_TAB_SETTINGS, ICON_TAB_SETTINGS_SELECTED};
use crate::core::tab::{TabView, TabViewSelector};
use crate::types::{FigureStyle, StyledLine, TextStyle};
use crate::values::{HEADER_OFFSET, HEADER_WIDTH};
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::{FONT_7X13_BOLD, FONT_7X14, FONT_7X14_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Primitive, Size};
use embedded_graphics::primitives::{CornerRadii, Line, PrimitiveStyle, Rectangle, RoundedRectangle};
use embedded_layout::layout::linear::FixedMargin;

pub const AREA: u32 = 16;
pub const RADIUS: u32 = AREA / 2;
pub const RADIUS_MINI: u32 = RADIUS / 2;
pub const OFFSET: i32 = 2;
pub const VISUAL_BASELINE_14: i32 = FONT_7X14.baseline as i32 + 1;
const TAB_SPACING: u32 = 2;
const TAB_COUNT: u32 = 2;
const TAB_WIDTH: u32 = (HEADER_WIDTH + TAB_SPACING) / TAB_COUNT - TAB_SPACING;
const HEADER_SPACE: i32 = 4;

pub const WHITE_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14, BinaryColor::On);
pub const BLACK_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14, BinaryColor::Off);
pub const BLACK_BOLD_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14_BOLD, BinaryColor::Off);
pub const WHITE_FILL: FigureStyle = PrimitiveStyle::with_fill(BinaryColor::On);
pub const BLACK_FILL: FigureStyle = PrimitiveStyle::with_fill(BinaryColor::Off);
pub const WHITE_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
const INVISIBLE_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::Off, 0);

pub const BATTERY_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);

pub const TAB_MARGIN: FixedMargin = FixedMargin(TAB_SPACING as i32);
pub const TEXT_ICON_MARGIN: FixedMargin = FixedMargin(2);

pub const HEADER_POINT: Point = Point::new(HEADER_OFFSET, 0);
pub const FIRST_TAB_POINT: Point = Point::new(HEADER_OFFSET, 0);
pub const SECOND_TAB_POINT: Point = Point::new(HEADER_OFFSET + TAB_WIDTH as i32 + HEADER_SPACE, 0);
pub const HEADER_SIZE: Size = Size::new(HEADER_WIDTH, AREA);
const CORNER: Size = Size::new(RADIUS, RADIUS);
const CORNER_MINI: Size = Size::new(RADIUS_MINI, RADIUS_MINI);
pub const CORNER_RADII: CornerRadii = CornerRadii::new(Size::new(RADIUS, RADIUS));
pub const FIRST_TAB_CORNER_RADII: CornerRadii = CornerRadii {
    top_left: CORNER,
    top_right: CORNER_MINI,
    bottom_right: CORNER_MINI,
    bottom_left: CORNER,
};
pub const LAST_TAB_CORNER_RADII: CornerRadii = CornerRadii {
    top_left: CORNER_MINI,
    top_right: CORNER,
    bottom_right: CORNER,
    bottom_left: CORNER_MINI,
};
pub const HEADER_RECTANGLE: RoundedRectangle = RoundedRectangle::new(Rectangle::new(HEADER_POINT, HEADER_SIZE), CORNER_RADII);
pub const TAB_RECTANGLE: RoundedRectangle = RoundedRectangle::new(Rectangle::new(Point::new(0, 0), Size::new(TAB_WIDTH, AREA)), CORNER_RADII);
pub const FIRST_TAB_RECTANGLE: RoundedRectangle = RoundedRectangle::new(Rectangle::new(FIRST_TAB_POINT, Size::new(TAB_WIDTH, AREA)), FIRST_TAB_CORNER_RADII);
pub const SECOND_TAB_RECTANGLE: RoundedRectangle = RoundedRectangle::new(Rectangle::new(SECOND_TAB_POINT, Size::new(TAB_WIDTH, AREA)), LAST_TAB_CORNER_RADII);
pub const CATHODE: Line = Line::new(Point::new(0, 0), Point::new(0, 4));
pub const BATTERY_CELL: RoundedRectangle = RoundedRectangle::new(Rectangle::new(Point::new(0, 3), Size::new(21, 13)), CornerRadii::new(Size::new(4, 4)));

pub const TAB_SETTINGS: TabViewSelector = TabViewSelector {
    normal: TabView::new(WHITE_STROKE, ICON_TAB_SETTINGS),
    selected: TabView::new(WHITE_FILL, ICON_TAB_SETTINGS_SELECTED),
};
pub const TAB_PUFFS: TabViewSelector = TabViewSelector {
    normal: TabView::new(WHITE_STROKE, ICON_TAB_PUFFS),
    selected: TabView::new(WHITE_FILL, ICON_TAB_PUFFS_SELECTED),
};

pub fn space(size: u32) -> StyledLine {
    Line::new(Point::new(0, 0), Point::new(size as i32 - 1, 0))
        .into_styled(INVISIBLE_STROKE)
}
