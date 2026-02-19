use crate::core::graphics::{AREA, BLACK_FILL, OFFSET, WHITE_FILL};
use crate::ext::result_ext::ResultExt;
use crate::types::Display;
use crate::values::SCREEN_WIDTH;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;

pub trait Cleaner {
    fn clear_header(&mut self, white: bool);
    fn clear_top_body(&mut self);
    fn clear_bottom_body(&mut self);
    fn clear_footer(&mut self);
}

impl Cleaner for Display {

    fn clear_header(&mut self, white: bool) {
        clear(self, 0, white)
    }

    fn clear_top_body(&mut self) {
        clear(self, AREA as i32 + OFFSET, false)
    }

    fn clear_bottom_body(&mut self) {
        clear(self, 2 * AREA as i32 + OFFSET, false)
    }

    fn clear_footer(&mut self) {
        clear(self, 3 * AREA as i32 + OFFSET, false)
    }
}

pub fn clear(display: &mut Display, offset: i32, white: bool) {
    Rectangle::new(Point::new(0, offset), Size::new(SCREEN_WIDTH, AREA))
        .into_styled(if white { WHITE_FILL } else { BLACK_FILL })
        .draw(display)
        .ignore()
}
