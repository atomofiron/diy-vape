use crate::core::graphics::HEADER_HEIGHT;
use crate::core::ui::place::Place;

pub trait Bounding {
    fn offset(&self) -> i32;
    fn height(&self) -> u32;
}

impl Bounding for Place {

    fn offset(&self) -> i32 {
        match self {
            Place::Top => 2,
            Place::Middle => 1,
            Place::Bottom => 2,
        }
    }

    fn height(&self) -> u32 {
        match self {
            Place::Top => HEADER_HEIGHT,
            Place::Middle => HEADER_HEIGHT,
            Place::Bottom => HEADER_HEIGHT - 2,
        }
    }
}
