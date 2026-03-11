#![no_std]
#![no_main]

use core::cmp::min;
use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use nrf52840_hal::gpio::p0::Parts as Parts0;
use nrf52840_hal::gpio::p1::Parts as Parts1;
use nrf52840_hal::gpio::{DriveConfig, Floating, Level, PullUp, PushPull};
use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::pwm::{Channel, Prescaler, Pwm};
use nrf52840_hal::rng::Rng;
use nrf52840_hal::twim::Pins as TwimPins;
use nrf52840_hal::twim::Twim;
#[allow(unused_imports)]
use panic_reset;
use ssd1306::command::AddrMode;
use ssd1306::prelude::*;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use vape::core::charge::Charge;
use vape::core::renderer::Renderer;
use vape::core::timer::Timer;
use vape::data::config::Config;
use vape::data::mode::Mode;
use vape::data::state::State;
use vape::data::stats::Stats;
use vape::ext::led_ext::LedExt;
use vape::ext::option_ext::{OptionExt, OptionOptionExt};
use vape::ext::result_ext::ResultExt;
use vape::flash::flash::AsyncFlash;
use vape::flash::flash_storage::FlashStorage;
use vape::flash::savable::Savable;
use vape::games::life::life::draw_life;
use vape::types::{Display, Duty, PinIn, PinOut, Time};
use vape::util::blocking::blocking;
use vape::util::logging::SoftUnwrap;
use vape::values::{BATTERY_PERIOD, IDLE_PERIOD, SCREENSAVER_TIMEOUT, SLEEP_PERIOD};

const ZERO_DUTY: Duty = 0;
const TEST_DUTY: Duty = 0x1;
const LOW_DUTY: Duty = 0x1fff;
const HALF_DUTY: Duty = 0x3fff;
const MAX_DUTY: Duty = 0x7fff;

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

    let mut touch = port1.p1_12.into_floating_input().degrade();
    let mut left_btn = port0.p0_03.into_pullup_input().degrade();
    let mut right_btn = port0.p0_28.into_pullup_input().degrade();

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

    let mut was_touched = false;
    let mut last_action = timer.now();
    loop {
        let touched = touch.is_high().unwrap_or(false);
        let left_pressed = left_btn.is_low().unwrap_or(false);
        let right_pressed = right_btn.is_low().unwrap_or(false);
        if !was_touched && touched {
            green.on();
            if state.is_display_on {
                state.next_mode();
            }
        } else if was_touched && !touched {
            green.off()
        }
        let interaction = was_touched != touched || state.buttons != (left_pressed, right_pressed);
        was_touched = touched;
        if interaction {
            last_action = timer.now();
        }
        if !state.is_display_on {
            match interaction {
                true => set_display(&mut state, &mut display, true),
                false => timer.sleep_ms(SLEEP_PERIOD as u32),
            }
            continue
        }

        let now = timer.now();
        handle_pressed(&mut state, left_pressed, right_pressed, now)
            .if_some(|new| pwm.set_duty_off(Channel::C0, *new));

        if touched || left_pressed || right_pressed || (now - last_action) < SCREENSAVER_TIMEOUT {
        } else if state.battery_charging {
            was_touched = screen_saver(&mut display, &mut rng, &mut touch, &mut left_btn, &mut right_btn, &mut green, now);
            last_action = timer.now();
            state.mark_all_dirty();
        } else {
            set_display(&mut state, &mut display, false);
        }
        update_battery(&mut charge, &mut timer, &mut state);
        state.render_dirty(&mut display);
        timer.sleep_ms(IDLE_PERIOD as u32);
    }
}

fn handle_pressed(
    state: &mut State,
    left_pressed: bool,
    right_pressed: bool,
    now: u64,
) -> Option<Duty> {
    let mut new_duty = None;
    match state.mode.clone() {
        Mode::Work { mut duration, prev, cool_down } => {
            let limit = state.limit_ms();
            let cool_down = duration >= limit || cool_down && (duration > 0 || left_pressed || right_pressed);
            match cool_down || !left_pressed || !right_pressed {
                true => duration -= min(now - prev, duration),
                _ if !state.buttons.0 => (),
                _ if !state.buttons.1 => (),
                _ => duration += now - prev,
            }
            state.set_work(duration, now, cool_down);
            if cool_down || !left_pressed || !right_pressed {
                new_duty = Some(ZERO_DUTY);
            } else if left_pressed && right_pressed {
                new_duty = Some(TEST_DUTY)
            }
        }
        _ if state.buttons == (left_pressed, right_pressed) => (),
        _ if state.buttons.0 || state.buttons.1 => (),
        _ if left_pressed == right_pressed => (),
        Mode::Power if left_pressed => state.dec_power(),
        Mode::Power if right_pressed => state.inc_power(),
        Mode::Limit if left_pressed => state.dec_limit(),
        Mode::Limit if right_pressed => state.inc_limit(),
        Mode::Resistance if left_pressed => state.dec_resistance(),
        Mode::Resistance if right_pressed => state.inc_resistance(),
        _ => (),
    }
    state.set_pressed(left_pressed, right_pressed);
    return new_duty
}

fn set_display(
    state: &mut State,
    display: &mut Display,
    on: bool,
) {
    if on != state.is_display_on {
        state.is_display_on = on;
        display.set_display_on(on)
            .ignore();
    }
}

fn update_battery(
    charge: &mut Charge,
    timer: &mut Timer,
    state: &mut State,
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
    state.is_battery_dirty = true;
    state.is_resistance_or_watt_dirty = true;
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
}

fn is_charging() -> bool {
    false // todo check charging leds
}

fn screen_saver(
    display: &mut Display,
    rng: &mut Rng,
    touch: &mut PinIn<Floating>,
    left_btn: &mut PinIn<PullUp>,
    right_btn: &mut PinIn<PullUp>,
    green: &mut PinOut<PushPull>,
    now: Time,
) -> bool {
    let mut flag = true;
    let mut start = true;
    let mut touched = false;
    let mut left_pressed = false;
    let mut right_pressed = false;
    while !touched && !left_pressed && !right_pressed {
        match flag {
            true => green.on(),
            false => green.off(),
        }
        flag = !flag;
        draw_life(display, rng, false, start, now);
        start = false;
        touched = touch.is_high().unwrap_or(false);
        left_pressed = left_btn.is_low().unwrap_or(false);
        right_pressed = right_btn.is_low().unwrap_or(false);
    }
    green.off();
    display.set_addr_mode(AddrMode::Horizontal)
        .unwrap();
    return touched
}
