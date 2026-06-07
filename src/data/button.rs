#[derive(PartialEq)]
pub enum Button {
    Left,
    Right,
}

impl Button {

    pub fn from(left: bool, right: bool) -> Option<Button> {
        match (left, right) {
            (true, false) => Some(Button::Left),
            (false, true) => Some(Button::Right),
            _ => None,
        }
    }

    pub fn is_left(&self) -> bool {
        *self == Self::Left
    }

    pub fn is_right(&self) -> bool {
        *self == Self::Right
    }
}
