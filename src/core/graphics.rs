use crate::values::{HEADER_OFFSET, HEADER_WIDTH};
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::{FONT_7X13_BOLD, FONT_7X14, FONT_7X14_BOLD};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Primitive, Size};
use embedded_graphics::primitives::{CornerRadii, Line, PrimitiveStyle, Rectangle, RoundedRectangle, Styled};

pub const AREA: u32 = 16;
pub const RADIUS: u32 = AREA / 2;
pub const OFFSET: i32 = 2;
pub const VISUAL_BASELINE_14: i32 = FONT_7X14.baseline as i32 + 1;

type TextStyle<'l> = MonoTextStyle<'l, BinaryColor>;
type FigureStyle = PrimitiveStyle<BinaryColor>;
type StyledLine = Styled<Line, FigureStyle>;

pub const WHITE_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14, BinaryColor::On);
pub const BLACK_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14, BinaryColor::Off);
pub const BLACK_BOLD_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14_BOLD, BinaryColor::Off);
pub const WHITE_FILL: FigureStyle = PrimitiveStyle::with_fill(BinaryColor::On);
pub const BLACK_FILL: FigureStyle = PrimitiveStyle::with_fill(BinaryColor::Off);
pub const WHITE_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
const INVISIBLE_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::Off, 0);

pub const BATTERY_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);

pub const HEADER_POINT: Point = Point::new(HEADER_OFFSET, 0);
pub const HEADER_SIZE: Size = Size::new(HEADER_WIDTH, AREA);
pub const CORNER_RADII: CornerRadii = CornerRadii::new(Size::new(RADIUS, RADIUS));
pub const HEADER_RECTANGLE: RoundedRectangle = RoundedRectangle::new(Rectangle::new(HEADER_POINT, HEADER_SIZE), CORNER_RADII);
pub const CATHODE: Line = Line::new(Point::new(0, 0), Point::new(0, 4));
pub const BATTERY_CELL: RoundedRectangle = RoundedRectangle::new(Rectangle::new(Point::new(0, 3), Size::new(21, 13)), CornerRadii::new(Size::new(4, 4)));

pub fn space(size: u32) -> StyledLine {
    Line::new(Point::new(0, 0), Point::new(size as i32 - 1, 0))
        .into_styled(INVISIBLE_STROKE)
}
