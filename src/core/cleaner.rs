use crate::ext::result_ext::ResultExt;
use crate::types::Display;
use crate::values::SCREEN_WIDTH;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::Drawable;

pub trait Cleaner {
    fn area_height() -> u32 { 16 }
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
        clear(self, Self::area_height() as i32)
    }

    fn clear_bottom_body(&mut self) {
        clear(self, 2 * Self::area_height() as i32)
    }

    fn clear_footer(&mut self) {
        clear(self, 3 * Self::area_height() as i32)
    }
}

pub fn clear(display: &mut Display, offset: i32) {
    Rectangle::new(Point::new(0, offset), Size::new(SCREEN_WIDTH, Display::area_height()))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
        .draw(display)
        .ignore()
}
