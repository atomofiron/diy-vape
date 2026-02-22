use cortex_m::asm::delay;
use embedded_hal::digital::OutputPin;
use hal::saadc::{Gain, Oversample, Reference, Resolution, Saadc, SaadcConfig};
use nrf52840_hal as hal;
use nrf52840_hal::gpio::{Output, PushPull};
use nrf52840_hal::pac::POWER;
use nrf52840_hal::pac::SAADC;

pub type Pin0_14 = hal::gpio::p0::P0_14<Output<PushPull>>;
pub type Pin0_31 = hal::gpio::p0::P0_31<hal::gpio::Input<hal::gpio::Floating>>;

pub struct Charge {
    pub vbat_en: Pin0_14,
    pub vbat_pin: Pin0_31,
    pub saadc: Saadc,
    pub power: POWER,
    pub last_check: u64,
}

impl Charge {

    pub fn init(p14: Pin0_14, p31: Pin0_31, saadc: SAADC, power: POWER) -> Charge {
        let mut cfg = SaadcConfig::default();
        cfg.resolution = Resolution::_12BIT;
        cfg.oversample = Oversample::OVER8X;
        cfg.reference = Reference::INTERNAL; // 0.6V
        cfg.gain = Gain::GAIN1_6; // full-scale ~3.6V
        Charge {
            vbat_en: p14,
            vbat_pin: p31,
            saadc: Saadc::new(saadc, cfg),
            power,
            last_check: 0,
        }
    }

    pub fn is_usb_connected(&self) -> bool {
        self.power.usbregstatus.read()
            .vbusdetect()
            .bit_is_set()
    }

    pub fn get_mv(&mut self) -> Option<u8> {
        if self.is_usb_connected() {
            return None;
        }
        self.enable_measuring();
        delay(1_000_000);
        let mut arr = [0i32; 8];
        arr.iter_mut()
            .for_each(|v| *v = self.read_mv() as i32);
        self.disable_measuring();
        let avg = arr.iter().sum::<i32>() / arr.len() as i32;
        let mut percents = (avg - 3500) / 7;
        percents = match () {
            _ if percents < 0 => 0,
            _ if percents > 100 => 100,
            _ => percents,
        };
        return Some(percents as u8);
    }

    pub fn read_mv(&mut self) -> u16 {
        let raw: i16 = self.saadc.read_channel(&mut self.vbat_pin)
            .unwrap_or(0)
            .max(0);
        // 12-bit: 0..4095
        // Reference::INTERNAL (0.6V) + Gain::GAIN1_6 => Vref/gain = 0.6 / (1/6) = 3.6V
        // V_adc_pin(mV) = raw * 3600 / 4096
        let vadc_mv = (raw as u32) * 3600u32 / 4096u32;
        // ~1/3 => VBAT ≈ Vadc * 3
        (vadc_mv * 3) as u16
    }

    fn enable_measuring(&mut self) -> bool {
        self.vbat_en.set_low().is_ok()
    }

    fn disable_measuring(&mut self) -> bool {
        self.vbat_en.set_high().is_ok()
    }
}


