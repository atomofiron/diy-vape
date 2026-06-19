use embedded_graphics::geometry::Point;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::BinaryColor;

pub trait ImageRawExt<'r> {
    fn to_image<'i>(&'i self) -> Image<'i, ImageRaw<'r, BinaryColor>>;
    fn to_image_on<'i>(&'i self, x: i32, y: i32) -> Image<'i, ImageRaw<'r, BinaryColor>>;
}

impl<'r> ImageRawExt<'r> for ImageRaw<'r, BinaryColor> {

    fn to_image<'i>(&'i self) -> Image<'i, ImageRaw<'r, BinaryColor>> {
        self.to_image_on(0, 0)
    }

    fn to_image_on<'i>(&'i self, x: i32, y: i32) -> Image<'i, ImageRaw<'r, BinaryColor>> {
        Image::new(self, Point::new(x, y))
    }
}
