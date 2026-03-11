use crate::types::Time;

#[derive(Clone)]
pub enum Mode {
    Work {
        duration: Time,
        prev: Time,
        cool_down: bool,
    },
    Power,
    Limit,
    Resistance,
}

impl Mode {

    pub fn next(&self) -> Mode {
        match self {
            Self::Work { .. }  => Mode::Power,
            Self::Power => Mode::Limit,
            Self::Limit => Mode::Resistance,
            Self::Resistance => Mode::default(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Work {
            prev: 0,
            duration: 0,
            cool_down: false,
        }
    }
}
