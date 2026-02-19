use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::{FONT_7X13_BOLD, FONT_7X14};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{Line, Polyline, PrimitiveStyle, Styled};

pub const AREA: u32 = 16;
pub const RADIUS: u32 = AREA / 2;
pub const OFFSET: i32 = 2;
pub const VISUAL_BASELINE_14: i32 = FONT_7X14.baseline as i32 + 1;

type TextStyle<'l> = MonoTextStyle<'l, BinaryColor>;
type FigureStyle = PrimitiveStyle<BinaryColor>;
type StyledPolyline<'l> = Styled<Polyline<'l>, FigureStyle>;
type StyledLine = Styled<Line, FigureStyle>;

pub const WHITE_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14, BinaryColor::On);
pub const BLACK_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X14, BinaryColor::Off);
pub const WHITE_FILL: FigureStyle = PrimitiveStyle::with_fill(BinaryColor::On);
pub const WHITE_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
pub const BLACK_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);
const INVISIBLE_STROKE: FigureStyle = PrimitiveStyle::with_stroke(BinaryColor::Off, 0);

pub const BATTERY_TEXT: TextStyle = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);

// ˟
const CROSS_POINTS: [Point; 5] = [Point::new(0, 0), Point::new(4, 4), Point::new(2, 2), Point::new(0, 4), Point::new(4, 0)];
pub const ICON_CROSS: StyledPolyline = Styled::new(Polyline::new(&CROSS_POINTS), WHITE_STROKE);

// Ω
const OHM_POINTS: [Point; 12] = [Point::new(0, 9), Point::new(2, 9), Point::new(2, 7), Point::new(0, 5), Point::new(0, 3), Point::new(3, 0), Point::new(5, 0), Point::new(8, 3), Point::new(8, 5), Point::new(6, 7), Point::new(6, 9), Point::new(8, 9)];
pub const ICON_OHM: StyledPolyline = Styled::new(Polyline::new(&OHM_POINTS), WHITE_STROKE);

// ⚡
const LIGHTNING_POINTS: [Point; 7] = [Point::new(3, 0), Point::new(0, 7), Point::new(5, 4), Point::new(2, 11), Point::new(2, 4), Point::new(3, 7), Point::new(3, 0)];
pub const ICON_CHARGING: Polyline = Polyline::new(&LIGHTNING_POINTS);

pub fn space(size: u32) -> StyledLine {
    Line::new(Point::new(0, 0), Point::new(size as i32 - 1, 0))
        .into_styled(INVISIBLE_STROKE)
}
