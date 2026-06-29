use crate::data::edit_settings::EditSettings;
use crate::data::reset_puffs::ResetPuffs;
use crate::types::{Duty, Progress, Time};
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
    Puffs(ResetPuffs, Option<Progress>),
    Battery,
}

impl Mode {

    pub fn settings() -> Self {
        Self::Settings(EditSettings::None)
    }

    pub fn puffs() -> Self {
        Self::Puffs(ResetPuffs::None, None)
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
