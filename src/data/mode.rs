use crate::data::edit_settings::EditSettings;
use crate::data::reset_puffs::ResetPuffs;
use crate::types::{Duty, Time};
use strum::EnumIs;

#[derive(Clone, PartialEq, EnumIs)]
pub enum Mode {
    Work {
        duration: Time,
        prev: Time,
        cool_down: bool,
        start: Option<Time>, // (start == Some) == Adc.measuring
        duty: Option<Duty>, // (duty == Some) != (start == Some), i.e. measurement completed
    },
    Settings(EditSettings),
    Puffs(ResetPuffs),
    Battery,
}

impl Mode {

    pub fn next(&self) -> Mode {
        match self {
            Self::Work { .. } => Mode::settings(),
            Self::Settings(EditSettings::None) => Mode::Settings(EditSettings::Power),
            Self::Settings(EditSettings::Power) => Mode::Settings(EditSettings::Limit),
            Self::Settings(EditSettings::Limit) => Mode::Settings(EditSettings::Resistance),
            Self::Settings(EditSettings::Resistance) => Mode::Settings(EditSettings::Brightness),
            Self::Settings(EditSettings::Brightness) => Mode::default(),
            Self::Puffs(ResetPuffs::None) => Mode::Puffs(ResetPuffs::Coil),
            Self::Puffs(ResetPuffs::Coil) => Mode::Puffs(ResetPuffs::All),
            Self::Puffs(ResetPuffs::All) => Mode::default(),
            Self::Battery => Mode::default(),
        }
    }

    pub fn settings() -> Mode {
        Self::Settings(EditSettings::None)
    }

    pub fn puffs() -> Mode {
        Self::Puffs(ResetPuffs::None)
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
