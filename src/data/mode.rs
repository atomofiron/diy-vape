use crate::data::tab::Tab;
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
    Tabs(Tab), // selected

    Power,
    Limit,
    Resistance,
    Brightness,

    ResetCoil,
    ResetStats,
}

impl Mode {

    pub fn next(&self) -> Mode {
        match self {
            Self::Tabs(tab) => match tab {
                Tab::Settings => Mode::Power,
                Tab::Puffs => Mode::ResetCoil,
                Tab::Battery => Mode::default(),
            }
            Self::Work { .. } => Mode::Tabs(Tab::Settings),
            Self::Power => Mode::Limit,
            Self::Limit => Mode::Resistance,
            Self::Resistance => Mode::Brightness,
            Self::Brightness => Mode::default(),

            Self::ResetCoil => Self::ResetStats,
            Self::ResetStats => Mode::default(),
        }
    }

    pub fn is_settings(&self) -> bool {
        match self {
            Self::Tabs(Tab::Settings) |
            Self::Power |
            Self::Limit |
            Self::Resistance |
            Self::Brightness => true,
            _ => false,
        }
    }

    pub fn is_puffs(&self) -> bool {
        match self {
            Self::Tabs(Tab::Puffs) |
            Self::ResetCoil |
            Self::ResetStats => true,
            _ => false,
        }
    }

    pub fn is_battery(&self) -> bool {
        match self {
            Self::Tabs(Tab::Battery) => true,
            _ => false,
        }
    }

    pub fn is_work(&self) -> bool {
        match self {
            Mode::Work { .. } => true,
            _ => false,
        }
    }

    pub fn is_brightness(&self) -> bool {
        *self == Mode::Brightness
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
