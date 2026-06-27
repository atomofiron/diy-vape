
#[derive(Clone, PartialEq)]
pub enum ChargeStatus {
    Discharging, // ''
    Charging, // '⚡'
    Full, // '⚡' (inverted)
    Reverse, // '↩'
    Unknown, // '!' intermediate state, no battery, etc.
}

impl Default for ChargeStatus {

    fn default() -> Self {
        ChargeStatus::Discharging
    }
}

impl ChargeStatus {

    pub fn pick(charging: bool, full: bool, reverse: bool) -> ChargeStatus {
        if charging && reverse || reverse && full {
            return ChargeStatus::Unknown;
        }
        match (charging, full, reverse) {
            (_, true, _) => ChargeStatus::Full,
            (true, _, _) => ChargeStatus::Charging,
            (_, _, true) => ChargeStatus::Reverse,
            _ => ChargeStatus::Discharging,
        }
    }

    pub fn is_powered(&self) -> bool {
        match self {
            ChargeStatus::Unknown |
            ChargeStatus::Reverse |
            ChargeStatus::Discharging => false,
            ChargeStatus::Charging |
            ChargeStatus::Full => true,
        }
    }
}
