use crate::core::graphics::{BLACK_FILL, HEADER_HEIGHT, RADIUS, WHITE_FILL, WHITE_STROKE};
use crate::data::buttons::Buttons;
use crate::ext::result_ext::ResultExt;
use crate::types::Display;
use crate::values::SCREEN_WIDTH;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::Circle;
use embedded_graphics::Drawable;

impl Buttons {

    pub fn render(&self, old: &Buttons, display: &mut Display) {
        let delta = RADIUS as i32 - 1;
        if self.left != old.left {
            let left_center = Point::new(delta, delta);
            if !self.left {
                Circle::with_center(left_center, HEADER_HEIGHT)
                    .into_styled(BLACK_FILL)
                    .draw(display)
                    .ignore();
            }
            Circle::with_center(left_center, HEADER_HEIGHT)
                .into_styled(if self.left { WHITE_FILL } else { WHITE_STROKE })
                .draw(display)
                .ignore();
        }
        if self.right != old.right {
            let right_center = Point::new((SCREEN_WIDTH - RADIUS) as i32 - 1, delta);
            if !self.right {
                Circle::with_center(right_center, HEADER_HEIGHT)
                    .into_styled(BLACK_FILL)
                    .draw(display)
                    .ignore();
            }
            Circle::with_center(right_center, HEADER_HEIGHT)
                .into_styled(if self.right { WHITE_FILL } else { WHITE_STROKE })
                .draw(display)
                .ignore();
        }
    }
}