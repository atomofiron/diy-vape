#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use nrf52840_hal::gpio::p0::Parts as Parts0;
use nrf52840_hal::gpio::p1::Parts as Parts1;
use nrf52840_hal::gpio::{DriveConfig, Level};
use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::pwm::{Channel, Prescaler, Pwm};
use nrf52840_hal::rng::Rng;
use nrf52840_hal::twim::Pins as TwimPins;
use nrf52840_hal::twim::Twim;
#[allow(unused_imports)]
use panic_reset;
use ssd1306::prelude::*;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use vape::core::charge::Charge;
use vape::core::renderer::Renderer;
use vape::core::timer::Timer;
use vape::data::config::Config;
use vape::data::mode::Mode;
use vape::data::state::State;
use vape::data::stats::Stats;
use vape::ext::option_ext::OptionExt;
use vape::ext::pin_ext::LedExt;
use vape::flash::flash::AsyncFlash;
use vape::flash::flash_storage::FlashStorage;
use vape::flash::savable::Savable;
use vape::games::life::life::alive;
use vape::types::Display;
use vape::util::blocking::blocking;
use vape::util::logging::SoftUnwrap;
use vape::values::BATTERY_PERIOD;

const ZERO_DUTY: u16 = 0;
const TEST_DUTY: u16 = 0x1;
const LOW_DUTY: u16 = 0x1fff;
const HALF_DUTY: u16 = 0x3fff;
const MAX_DUTY: u16 = 0x7fff;

#[entry]
fn main() -> ! {
    // fake async
    let _ = blocking(async {
        async_main().await
    });
    loop {}
}

async fn async_main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let port0 = Parts0::new(peripherals.P0);
    let port1 = Parts1::new(peripherals.P1);

    let mut touch = port1.p1_12.into_pulldown_input();

    let mut red = port0.p0_26.into_push_pull_output(Level::High).degrade();
    let mut green = port0.p0_30.into_push_pull_output(Level::High).degrade();
    let mut blue = port0.p0_06.into_push_pull_output(Level::High).degrade();

    blue.blink();

    let pin_02 = port0.p0_02.into_push_pull_output_drive(Level::Low, DriveConfig::HighDrive0HighDrive1).degrade();
    let pwm = Pwm::new(peripherals.PWM0);
    pwm.set_output_pin(Channel::C0, pin_02);
    pwm.set_prescaler(Prescaler::Div1);
    pwm.set_max_duty(MAX_DUTY);
    pwm.enable();

    //let mut gate = port0.p0_02.into_push_pull_output_drive(Level::Low, DriveConfig::HighDrive0HighDrive1);
    //let mut gate = port0.p0_02.into_push_pull_output(Level::Low);

    let mut timer = Timer::init(peripherals.RTC1, peripherals.CLOCK);

    let scl = port0.p0_05.into_floating_input()
        .degrade();
    let sda = port0.p0_04.into_floating_input()
        .degrade();

    let pins = TwimPins { scl, sda };
    let i2c = Twim::new(peripherals.TWIM0, pins, nrf52840_hal::twim::Frequency::K400);
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    if display.init().is_err() {
        red.on();
    }

    let flash = AsyncFlash::from(peripherals.NVMC);
    let mut storage = flash.storage();

    let mut buf = [0u8; Config::FLASH_BUFFER_SIZE];
    let config =storage.read::<Config>(&mut buf)
        .await.soft_unwrap()
        .flat()
        .unwrap_or_default();

    let mut buf = [0u8; Stats::FLASH_BUFFER_SIZE];
    let stats = storage.read::<Stats>(&mut buf)
        .await.soft_unwrap()
        .flat()
        .unwrap_or_default();

    let mut charge = Charge::init(
        port0.p0_14.into_push_pull_output(Level::High),
        port0.p0_31.into_floating_input(),
        peripherals.SAADC,
        peripherals.POWER,
    );

    let mut rng = Rng::new(peripherals.RNG);

    let mut state = State::with(config, stats);
    state.render_all(&mut display);

    green.blink();

    let mut touched = false;
    loop {
        let now = touch.is_high().unwrap_or(false);
        match now {
            _ if now == touched && now && matches!(state.mode, Mode::Work(_)) => {
                state.inc_progress();
                state.render_header(&mut display);
                match &state.mode {
                    Mode::Work(255) => {
                        green.off();
                        pwm.set_duty_off(Channel::C0, ZERO_DUTY);
                        alive(&mut display, &mut timer, &mut rng, false, &mut green)
                    },
                    _ => (),
                }
            },
            _ if now == touched => (),
            true => {
                //gate.set_high().ignore();
                pwm.set_duty_off(Channel::C0, TEST_DUTY);
                green.on();
                state.next_mode();
                state.render_all(&mut display);
            },
            false => {
                //gate.set_low().ignore();
                pwm.set_duty_off(Channel::C0, ZERO_DUTY);
                green.off();
                state.reset_progress();
                state.render_header(&mut display);
            },
        }
        touched = now;
        timer.sleep_ms(10);
        update_battery(&mut charge, &mut timer, &mut state, &mut display);
    }
}

fn update_battery(
    charge: &mut Charge,
    timer: &mut Timer,
    state: &mut State,
    display: &mut Display,
) {
    let now = timer.now();
    let connected = charge.is_usb_connected();
    let is_charging = is_charging() || connected; // todo remove '|| connected'
    let need_update = state.battery_level.is_none() || charge.last_check == 0 || (now - charge.last_check) > BATTERY_PERIOD;
    match () {
        _ if connected != state.usb_connected => (),
        _ if is_charging != state.battery_charging => (),
        _ if !connected && need_update => (),
        _ => return,
    }
    state.usb_connected = connected;
    state.battery_charging = is_charging;
    if !connected && need_update {
        charge.last_check = now;
        let mv_and_level = charge.get_mv_and_level();
        state.battery_level = mv_and_level.map(|(_, level)| level);
        state.battery_voltage = mv_and_level.map(|(voltage, _)| voltage);
    } else if connected { // don't measure battery level if usb is connected to the nrf52840
        state.battery_level = None;
        state.battery_voltage = None;
    }
    state.render_resistance_and_watt(display);
    state.render_footer(display);
}

fn is_charging() -> bool {
    false // todo check charging leds
}
