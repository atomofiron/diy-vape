use crate::core::cleaner::Cleaner;
use crate::core::ui::header::Header;
use crate::core::ui::place::Place;
use crate::core::ui::widget::Widget;
use crate::data::buttons::Buttons;
use crate::ext::result_ext::ResultExt;
use crate::types::Display;

mod buttons;
mod placing;

pub mod header;
pub mod bouding;
pub mod place;
pub mod power_and_limit;
pub mod tab;
pub mod widget;

#[derive(Clone, PartialEq)]
pub struct Ui {
    pub buttons: Buttons,
    pub header: Header,
    pub top: Widget,
    pub middle: Widget,
    pub bottom: Widget,
}

impl Ui {

    pub fn render(&self, old: &Ui, display: &mut Display) {
        if self == old {
            return
        }
        let old_buttons = match old.header {
            Header::None => &Buttons::new(!self.buttons.left, !self.buttons.right),
            _ => &old.buttons,
        };
        if self.buttons != *old_buttons {
            self.buttons.render(old_buttons, display);
        }
        if self.header != old.header {
            display.clear_header(self.header.is_title());
            self.header.render(display);
        }
        if self.top != old.top {
            display.clear_place(Place::Top);
            self.top.render(display);
        }
        if self.middle != old.middle {
            display.clear_place(Place::Middle);
            self.middle.render(display);
        }
        if self.bottom != old.bottom {
            display.clear_place(Place::Bottom);
            self.bottom.render(display);
        }
        display.flush().ignore();
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            header: Header::None,
            top: Widget::None,
            middle: Widget::None,
            bottom: Widget::None,
        }
    }
}
