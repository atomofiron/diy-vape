#[derive(Clone, PartialEq)]
pub struct Buttons {
    pub left: bool,
    pub right: bool,
}

impl Buttons {

    pub fn new(left: bool, right: bool) -> Self {
        Self { left, right }
    }
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}
