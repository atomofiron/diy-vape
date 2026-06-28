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

    pub fn next(&self) -> Self {
        match self {
            Self::Work { .. } => Self::settings(),
            Self::Settings(EditSettings::None) => Self::Settings(EditSettings::Power),
            Self::Settings(EditSettings::Power) => Self::Settings(EditSettings::Limit),
            Self::Settings(EditSettings::Limit) => Self::Settings(EditSettings::Resistance),
            Self::Settings(EditSettings::Resistance) => Self::Settings(EditSettings::Brightness),
            Self::Settings(EditSettings::Brightness) => Self::settings(),
            Self::Puffs(ResetPuffs::None) => Self::Puffs(ResetPuffs::Coil),
            Self::Puffs(ResetPuffs::Coil) => Self::Puffs(ResetPuffs::All),
            Self::Puffs(ResetPuffs::All) => Self::puffs(),
            Self::Battery => Self::Battery,
        }
    }

    pub fn settings() -> Self {
        Self::Settings(EditSettings::None)
    }

    pub fn puffs() -> Self {
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
