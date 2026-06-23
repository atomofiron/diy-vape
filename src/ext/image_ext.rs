use crate::types::{Icon, IconRaw};
use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;

pub trait IconRawExt<'r> {
    fn to_icon<'i>(&'i self) -> Icon<'i, 'r>;
    fn to_icon_on<'i>(&'i self, x: i32, y: i32) -> Icon<'i, 'r>;
}

impl<'r> IconRawExt<'r> for IconRaw<'r> {

    fn to_icon<'i>(&'i self) -> Icon<'i, 'r> {
        self.to_icon_on(0, 0)
    }

    fn to_icon_on<'i>(&'i self, x: i32, y: i32) -> Icon<'i, 'r> {
        Image::new(self, Point::new(x, y))
    }
}
