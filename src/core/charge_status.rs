
#[derive(PartialEq)]
pub enum ChargeStatus {
    Discharging, // '' initial status
    Charging, // '⚡' (fill)
    Full, // '⚡' (stroke)
    Unknown, // '!' intermediate state, no battery, etc.
}

impl Default for ChargeStatus {

    fn default() -> Self {
        ChargeStatus::Discharging
    }
}

impl ChargeStatus {

    pub fn pick(charging: bool, full: bool) -> ChargeStatus {
        match (charging, full) {
            (false, false) => ChargeStatus::Discharging,
            (true, false) => ChargeStatus::Charging,
            (false, true) => ChargeStatus::Full,
            (true, true) => ChargeStatus::Unknown,
        }
    }

    pub fn is_powered(&self) -> bool {
        match self {
            ChargeStatus::Discharging => false,
            ChargeStatus::Charging |
            ChargeStatus::Full |
            ChargeStatus::Unknown => true,
        }
    }
}
