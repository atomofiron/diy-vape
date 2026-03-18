use crate::core::charge_status::ChargeStatus;
use crate::types::{MilliVolt, Percent};

pub struct Battery {
    pub status: ChargeStatus, // 4056H
    pub level: Option<Percent>,
    pub idle: Option<MilliVolt>,
    pub load: Option<MilliVolt>,
}

impl Default for Battery {

    fn default() -> Self {
        Battery {
            status: ChargeStatus::default(),
            level: None,
            idle: None,
            load: None,
        }
    }
}
