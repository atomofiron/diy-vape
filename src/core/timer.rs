use nrf52840_hal as hal;
use nrf52840_hal::pac::{CLOCK, RTC1};
use nrf52840_hal::rtc::{RtcCompareReg, RtcInterrupt};
use nrf52840_hal::{pac, Rtc};

const RTC_1_KHZ: u32 = 31; // 1Hz ≈ 1ms
const MAX_COUNTER: u32 = 0xFFFFFF;

pub struct Timer {
    rtc: Rtc<RTC1>,
    last: u32,
    total: u64,
}

impl Timer {

    pub fn init(rtc: RTC1, clock: CLOCK) -> Timer {
        let clocks = hal::clocks::Clocks::new(clock);
        let _clocks = clocks.start_lfclk();
        let rtc = Rtc::new(rtc, RTC_1_KHZ).unwrap();
        rtc.enable_counter();
        return Timer { rtc, last: 0, total: 0 };
    }

    pub fn now(&mut self) -> u64 {
        self.update_total();
        return self.total
    }

    pub fn sleep_ms(&mut self, ms: u32) {
        let counter = self.rtc.get_counter();
        let target = (counter + ms) & MAX_COUNTER;
        self.update_total();

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

    fn update_total(&mut self) {
        let counter = self.rtc.get_counter();
        self.total += match () {
            _ if counter >= self.last => (counter - self.last) as u64,
            _ => (MAX_COUNTER - counter) as u64 + counter as u64,
        };
        self.last = counter;
    }
}