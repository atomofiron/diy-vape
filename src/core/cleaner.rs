use crate::core::graphics::{BLACK_FILL, HEADER_HEIGHT, HEADER_RECTANGLE, WHITE_FILL};
use crate::core::ui::bouding::Bounding;
use crate::core::ui::place::Place;
use crate::ext::result_ext::ResultExt;
use crate::types::Display;
use crate::values::SCREEN_WIDTH;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;

pub trait Cleaner {
    fn clear_header(&mut self, white: bool);
    fn clear_place(&mut self, place: Place);
}

impl Cleaner for Display {

    fn clear_header(&mut self, white: bool) {
        HEADER_RECTANGLE
            .into_styled(if white { WHITE_FILL } else { BLACK_FILL })
            .draw(self)
            .ignore();
    }

    fn clear_place(&mut self, place: Place) {
        let line = match place {
            Place::Top => 1,
            Place::Middle => 2,
            Place::Bottom => 3,
        };
        let y = line * HEADER_HEIGHT as i32 + place.offset();
        Rectangle::new(Point::new(0, y), Size::new(SCREEN_WIDTH, place.height()))
            .into_styled(BLACK_FILL)
            .draw(self)
            .ignore()
    }
}
