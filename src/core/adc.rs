use crate::types::{MilliVolt, Percent, Time};
use crate::util::logging::SoftUnwrap;
use crate::values::{VOLTS_MAX, VOLTS_MIN};
use cortex_m::asm::delay;
use embedded_hal::digital::OutputPin;
use hal::gpio::p0::{P0_14, P0_31};
use hal::gpio::{Floating, Input};
use hal::saadc::{Gain, Oversample, Reference, Resolution, Saadc, SaadcConfig};
use nrf52840_hal as hal;
use nrf52840_hal::gpio::{Output, PushPull};
use nrf52840_hal::pac::POWER;
use nrf52840_hal::pac::SAADC;

pub type Pin0_14 = P0_14<Output<PushPull>>;
pub type Pin0_31 = P0_31<Input<Floating>>;

pub struct Adc {
    pub vbat_en: Pin0_14,
    pub vbat_pin: Pin0_31,
    pub saadc: Saadc,
    pub power: POWER,
    pub last_check: u64,
}

impl Adc {

    pub fn init(p14: Pin0_14, p31: Pin0_31, raw_saadc: SAADC, power: POWER) -> Adc {
        let mut cfg = SaadcConfig::default();
        cfg.resolution = Resolution::_12BIT;
        cfg.oversample = Oversample::OVER8X;
        cfg.reference = Reference::INTERNAL; // 0.6V
        cfg.gain = Gain::GAIN1_6; // full-scale ~3.6V
        Adc {
            vbat_en: p14,
            vbat_pin: p31,
            saadc: Saadc::new(raw_saadc, cfg),
            power,
            last_check: 0,
        }
    }

    pub fn is_usb_connected(&self) -> bool {
        self.power.usbregstatus.read()
            .vbusdetect()
            .bit_is_set()
    }

    pub fn get_mv_and_level(&mut self, now: Time) -> Option<(MilliVolt, Percent)> {
        if self.is_usb_connected() {
            return None;
        }
        self.last_check = now;
        Self::calibrate();
        if !self.start_measuring() {
            return None;
        }
        delay(1_000_000);
        let mv = self.finish_measuring()?;
        let percents = (mv.max(VOLTS_MIN) - VOLTS_MIN) * 100 / (VOLTS_MAX - VOLTS_MIN);
        return Some((mv, percents.clamp(0, 100) as Percent))
    }

    pub fn start_measuring(&mut self) -> bool {
        self.vbat_en.set_low()
            .soft_unwrap().is_some()
    }

    pub fn finish_measuring(&mut self) -> Option<MilliVolt> {
        let mut arr = [0u32; 8];
        arr.iter_mut()
            .for_each(|v| *v = self.read_mv() as u32);
        self.vbat_en.set_high()
            .soft_unwrap();
        let avg = arr.iter().sum::<u32>() / arr.len() as u32;
        return Some(avg as MilliVolt);
    }

    pub fn read_mv(&mut self) -> MilliVolt {
        let raw = self.saadc.read_channel(&mut self.vbat_pin)
            .unwrap_or(0)
            .max(0);
        // 12-bit: 0..4095
        // Reference::INTERNAL (0.6V) + Gain::GAIN1_6 => Vref/gain = 0.6 / (1/6) = 3.6V
        // V_adc_pin(mV) = raw * 3600 / 4096
        let vadc_mv = (raw as u32) * 3600 / 4096;
        // ~1/3 => VBAT ≈ Vadc * 3
        (vadc_mv * 3) as MilliVolt
    }

    fn calibrate() {
        let saadc = unsafe { &*nrf52840_hal::pac::SAADC::ptr() };
        saadc.events_calibratedone.reset();
        saadc.tasks_calibrateoffset.write(|w| unsafe { w.bits(1) });
        while saadc.events_calibratedone.read().bits() == 0 {
            core::hint::spin_loop();
        }
        saadc.events_calibratedone.reset();
    }
}


