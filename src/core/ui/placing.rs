use crate::core::graphics::HEADER_HEIGHT;
use crate::core::ui::bouding::Bounding;
use crate::core::ui::place::Place;
use crate::types::Display;
use embedded_graphics::geometry::{Dimensions, Point};
use embedded_layout::align::HorizontalAlignment;
use embedded_layout::prelude::{horizontal, vertical, Align};
use embedded_layout::View;

pub trait Placing {
    fn place<A: HorizontalAlignment>(self, place: Place, alignment: A, display: &Display) -> Self;
    fn place_center(self, place: Place, display: &Display) -> Self;
}

impl<V: View> Placing for V {

    fn place<A: HorizontalAlignment>(self, place: Place, alignment: A, display: &Display) -> Self {
        let display_area = display.bounding_box();
        let area = HEADER_HEIGHT as i32;
        match place {
            Place::Top => self.align_to(&display_area, alignment, vertical::Center)
                .translate(Point::new(0, -area / 2 + place.offset())),
            Place::Middle => self.align_to(&display_area, alignment, vertical::Center)
                .translate(Point::new(0, area / 2 + place.offset())),
            Place::Bottom => self.align_to(&display_area, alignment, vertical::Bottom),
        }
    }

    fn place_center(self, place: Place, display: &Display) -> Self {
        self.place(place, horizontal::Center, display)
    }
}
