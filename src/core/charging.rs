use crate::ext::error::ErrorMessage;
use crate::types::{PinIn, Rslt};
use embedded_hal::digital::InputPin;
use nrf52840_hal::gpio::PullUp;

pub struct Charging {
    chrg: PinIn<PullUp>,
    stdby: PinIn<PullUp>,
}

impl Charging {

    pub fn new(
        chrg: PinIn<PullUp>,
        stdby: PinIn<PullUp>,
    ) -> Charging {
        Charging { chrg, stdby }
    }

    pub fn is_charging(&mut self) -> Rslt<bool> {
        let charging = self.chrg.is_low()
            .map_err(|_| ErrorMessage("charging check failed"))?;
        /*let stdby = self.stdby.is_low()
            .map_err(|_| ErrorMessage("charging stdby check failed"))?;*/
        return Ok(charging /*todo || stdby*/)
    }

    pub fn is_stdby(&mut self) -> Rslt<bool> {
        self.stdby.is_low()
            .map_err(|_| ErrorMessage("charging stdby check failed"))
    }
}
