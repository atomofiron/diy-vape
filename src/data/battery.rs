use crate::core::charge_status::ChargeStatus;
use crate::types::MilliVolt;

pub struct Battery {
    pub status: ChargeStatus, // 4056H
    pub idle: Option<MilliVolt>,
    pub load: Option<MilliVolt>,
}

impl Default for Battery {

    fn default() -> Self {
        Battery {
            status: ChargeStatus::default(),
            idle: None,
            load: None,
        }
    }
}

impl Battery {

    pub fn is_full(&self) -> bool {
        self.status == ChargeStatus::Full
    }
}
