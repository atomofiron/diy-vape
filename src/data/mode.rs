
pub enum Mode {
    Work(u8),
    Power,
    Limit,
    Resistance,
}

impl Mode {

    pub fn next(&self) -> Mode {
        match self {
            Self::Work(_) => Mode::Power,
            Self::Power => Mode::Limit,
            Self::Limit => Mode::Resistance,
            Self::Resistance => Mode::Work(0),
        }
    }
}
