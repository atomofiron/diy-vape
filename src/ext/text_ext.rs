use crate::values::SCREEN_WIDTH;
use embedded_graphics::mono_font::ascii::FONT_7X14;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle, Styled};
use embedded_graphics::text::Text;
use embedded_layout::View;

pub trait TextExt {
    fn center(self) -> Self;
    fn background(&self, selected: bool) -> Background;
}

type TextType<'a> = Text<'a, MonoTextStyle<'a, BinaryColor>>;
type Background = Styled<RoundedRectangle, PrimitiveStyle<BinaryColor>>;

impl<'a> TextExt for TextType<'a> {

    fn center(mut self) -> TextType<'a> {
        let width = self.bounding_box().size.width;
        let offset = (SCREEN_WIDTH - width) / 2;
        self.position.x = offset as i32;
        self
    }

    fn background(&self, selected: bool) -> Background {
        let size = self.size();
        let top = size.height - FONT_7X14.baseline;
        let left = top / 2 + top % 2;
        let right = top / 2;
        let size = Size::new(size.width + left + right, size.height);
        let color = match selected {
            true => BinaryColor::On,
            false => BinaryColor::Off,
        };
        RoundedRectangle::new(Rectangle::new(Point::new(-(left as i32), top as i32), size), CornerRadii::new(Size::new(5, 5)))
            .offset(1)
            .into_styled(PrimitiveStyle::with_fill(color))
    }
}