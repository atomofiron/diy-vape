use crate::core::charge_status::ChargeStatus;
use crate::types::MilliVolt;

#[derive(Clone, PartialEq)]
pub struct Battery {
    pub status: ChargeStatus,
    pub idle: Option<MilliVolt>,
    pub load: Option<MilliVolt>,
    pub full: Option<MilliVolt>,
}

impl Default for Battery {

    fn default() -> Self {
        Battery {
            status: ChargeStatus::default(),
            idle: None,
            load: None,
            full: None,
        }
    }
}

impl Battery {

    pub fn is_full(&self) -> bool {
        self.status == ChargeStatus::Full
    }
}
