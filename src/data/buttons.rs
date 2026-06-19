
pub struct Buttons {
    pub left: bool,
    pub right: bool,
}

impl Default for Buttons {
    fn default() -> Self {
        Buttons {
            left: false,
            right: false,
        }
    }
}
