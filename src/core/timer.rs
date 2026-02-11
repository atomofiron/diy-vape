use nrf52840_hal as hal;
use nrf52840_hal::pac::{CLOCK, RTC1};
use nrf52840_hal::rtc::{RtcCompareReg, RtcInterrupt};
use nrf52840_hal::{pac, Rtc};

const RTC_1_KHZ: u32 = 31; // 1Hz ≈ 1ms

pub struct Timer {
    rtc: Rtc<RTC1>,
}

impl Timer {

    pub fn init(rtc: RTC1, clock: CLOCK) -> Timer {
        let clocks = hal::clocks::Clocks::new(clock);
        let _clocks = clocks.start_lfclk();
        let rtc = Rtc::new(rtc, RTC_1_KHZ).unwrap();
        rtc.enable_counter();
        Timer { rtc }
    }

    pub fn now(&self) -> u32 {
        self.rtc.get_counter()
    }

    pub fn reset(&self) {
        self.rtc.clear_counter()
    }

    pub fn sleep_ms(&mut self, ms: u32) {
        let now = self.rtc.get_counter();
        let target = (now + ms) & 0x00FF_FFFF;

        self.rtc.set_compare(RtcCompareReg::Compare0, target).unwrap();
        self.rtc.reset_event(RtcInterrupt::Compare0);

        self.rtc.enable_event(RtcInterrupt::Compare0);
        self.rtc.enable_interrupt(RtcInterrupt::Compare0, None);

        unsafe {
            pac::NVIC::unpend(pac::Interrupt::RTC1);
            pac::NVIC::unmask(pac::Interrupt::RTC1);
            cortex_m::interrupt::disable();
        }
        while !self.rtc.is_event_triggered(RtcInterrupt::Compare0) {
            cortex_m::asm::wfi();
        }
        unsafe {
            pac::NVIC::mask(pac::Interrupt::RTC1);
            pac::NVIC::unpend(pac::Interrupt::RTC1);
            cortex_m::interrupt::enable();
        }
        self.rtc.reset_event(RtcInterrupt::Compare0);
        self.rtc.disable_event(RtcInterrupt::Compare0);
        self.rtc.disable_interrupt(RtcInterrupt::Compare0, None);
    }
}