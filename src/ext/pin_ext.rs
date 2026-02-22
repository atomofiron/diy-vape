use crate::ext::result_ext::ResultExt;
use crate::types::POPP;
use embedded_hal::digital::OutputPin;

pub trait LedExt {
    fn on(&mut self);
    fn off(&mut self);
    fn blink(&mut self);
}

impl LedExt for POPP {

    fn on(&mut self) {
        self.set_low().ignore();
        cortex_m::asm::delay(4_000_000);
    }

    fn off(&mut self) {
        self.set_high().ignore();
        cortex_m::asm::delay(4_000_000);
    }

    fn blink(&mut self) {
        self.on();
        self.off();
    }
}
