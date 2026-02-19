use crate::core::graphics::{AREA, OFFSET};
use crate::ext::result_ext::ResultExt;
use crate::types::Display;
use crate::values::SCREEN_WIDTH;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::Drawable;

pub trait Cleaner {
    fn clear_header(&mut self);
    fn clear_top_body(&mut self);
    fn clear_bottom_body(&mut self);
    fn clear_footer(&mut self);
}

impl Cleaner for Display {

    fn clear_header(&mut self) {
        clear(self, 0)
    }

    fn clear_top_body(&mut self) {
        clear(self, AREA as i32 + OFFSET)
    }

    fn clear_bottom_body(&mut self) {
        clear(self, 2 * AREA as i32 + OFFSET)
    }

    fn clear_footer(&mut self) {
        clear(self, 3 * AREA as i32 + OFFSET)
    }
}

pub fn clear(display: &mut Display, offset: i32) {
    Rectangle::new(Point::new(0, offset), Size::new(SCREEN_WIDTH, AREA))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(display)
        .ignore()
}
