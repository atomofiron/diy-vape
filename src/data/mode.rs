use crate::types::{Duty, Time};

#[derive(Clone, PartialEq)]
pub enum Mode {
    Work {
        duration: Time,
        prev: Time,
        cool_down: bool,
        start: Option<Time>, // (start == Some) == Adc.measuring
        duty: Option<Duty>, // (duty == Some) != (start == Some)
    },
    Power,
    Limit,
    Resistance,
    Brightness,
}

impl Mode {

    pub fn next(&self) -> Mode {
        match self {
            Self::Work { .. } => Mode::Power,
            Self::Power => Mode::Limit,
            Self::Limit => Mode::Resistance,
            Self::Resistance => Mode::Brightness,
            Self::Brightness => Mode::default(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Work {
            prev: 0,
            duration: 0,
            cool_down: false,
            start: None,
            duty: None,
        }
    }
}
