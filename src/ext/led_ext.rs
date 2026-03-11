use crate::ext::result_ext::ResultExt;
use crate::types::PinOut;
use embedded_hal::digital::OutputPin;

pub trait LedExt {
    fn on(&mut self);
    fn off(&mut self);
    fn blink(&mut self);
}

impl<M> LedExt for PinOut<M> {

    fn on(&mut self) {
        self.set_low().ignore();
    }

    fn off(&mut self) {
        self.set_high().ignore();
    }

    fn blink(&mut self) {
        self.on();
        cortex_m::asm::delay(4_000_000);
        self.off();
        cortex_m::asm::delay(4_000_000);
    }
}
