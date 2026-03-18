use crate::ext::error::ErrorMessage;
use crate::types::{PinIn, Rslt};
use embedded_hal::digital::InputPin;
use nrf52840_hal::gpio::Floating;

type Pin = PinIn<Floating>;

pub struct Charging {
    charging: Pin,
    standby: Pin,
}

impl Charging {

    pub fn new(charging: Pin, standby: Pin) -> Charging {
        Charging { charging, standby }
    }

    pub fn is_charging(&mut self) -> Rslt<bool> {
        self.charging.is_low()
            .map_err(|_| ErrorMessage("charging check failed"))
    }

    pub fn is_full(&mut self) -> Rslt<bool> {
        self.standby.is_low()
            .map_err(|_| ErrorMessage("charging standby check failed"))
    }
}
