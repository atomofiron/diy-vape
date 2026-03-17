#![no_std]
#![no_main]

use core::cmp::min;
use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use nrf52840_hal::gpio::p0::Parts as Parts0;
use nrf52840_hal::gpio::p1::Parts as Parts1;
use nrf52840_hal::gpio::{Floating, Level, PullUp, PushPull};
use nrf52840_hal::pac::Peripherals;
use nrf52840_hal::pwm::{Channel, Prescaler, Pwm};
use nrf52840_hal::rng::Rng;
use nrf52840_hal::twim::Twim;
use nrf52840_hal::twim::{Frequency, Pins as TwimPins};
use ssd1306::command::AddrMode;
use ssd1306::prelude::*;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use vape::core::adc::Adc;
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
use vape::types::{Display, Duty, MilliWatt, PinIn, PinOut, Time};
use vape::util::blocking::blocking;
use vape::util::logging::SoftUnwrap;
use vape::values::{BATTERY_PERIOD, DISPLAY_PRECHARGE, IDLE_PERIOD, SCREENSAVER_TIMEOUT, SLEEP_PERIOD, VOLTS_MAX};

const ZERO_DUTY: Duty = 0;
const TEST_DUTY: Duty = 0x4;
const MAX_DUTY: Duty = 0x7fff;

#[entry]
fn main() -> ! {
    // fake async
    blocking(async {
        bustle().await
    })
}

async fn bustle() -> ! {
    let peripherals = Peripherals::take()
        .unwrap();

    let port0 = Parts0::new(peripherals.P0);
    let port1 = Parts1::new(peripherals.P1);

    let mut touch = port1.p1_12.into_floating_input().degrade();
    let mut left_btn = port0.p0_03.into_pullup_input().degrade();
    let mut right_btn = port0.p0_28.into_pullup_input().degrade();

    let mut red = port0.p0_26.into_push_pull_output(Level::High).degrade();
    let mut green = port0.p0_30.into_push_pull_output(Level::High).degrade();
    let mut blue = port0.p0_06.into_push_pull_output(Level::High).degrade();

    blue.blink();

    let pin_02 = port0.p0_02.into_push_pull_output(Level::Low).degrade();
    let pwm = Pwm::new(peripherals.PWM0);
    pwm.set_output_pin(Channel::C0, pin_02);
    pwm.set_prescaler(Prescaler::Div1);
    pwm.set_max_duty(MAX_DUTY);
    pwm.enable();

    let mut timer = Timer::init(peripherals.RTC1, peripherals.CLOCK)
        .unwrap();

    let scl = port0.p0_05.into_floating_input()
        .degrade();
    let sda = port0.p0_04.into_floating_input()
        .degrade();

    let pins = TwimPins { scl, sda };
    let i2c = Twim::new(peripherals.TWIM0, pins, Frequency::K400);
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    if display.init().is_err() { // DisplayError is unaccessible >:(
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

    let mut adc = Adc::init(
        port0.p0_14.into_push_pull_output(Level::High),
        port0.p0_31.into_floating_input(),
        peripherals.SAADC,
        peripherals.POWER,
    );
    let mut rng = Rng::new(peripherals.RNG);
    let mut state = State::with(config, stats);

    green.blink();

    let mut was_touched = false;
    let mut last_interaction = timer.now();
    loop {
        let touched = touch.is_high().unwrap_or(false);
        let left_pressed = left_btn.is_low().unwrap_or(false);
        let right_pressed = right_btn.is_low().unwrap_or(false);
        if !was_touched && touched {
            green.on();
            if state.is_display_on && !left_pressed && !right_pressed {
                state.next_mode();
            }
        } else if was_touched && !touched {
            green.off()
        }
        let interaction = was_touched != touched
            || state.buttons != (left_pressed, right_pressed)
            || state.battery_charging != (is_charging() || adc.is_usb_connected()); // todo remove '|| connected'
        was_touched = touched;
        let mut now = timer.now();
        if interaction {
            last_interaction = now;
        }
        if !state.is_display_on {
            match interaction {
                true => set_display(&mut state, &mut display, true),
                false => timer.sleep_ms(SLEEP_PERIOD as u32)
                    .unwrap_or_else(|_| red.blink()),
            }
            continue
        }

        handle_pressed(&mut state, &mut adc, left_pressed, right_pressed, now);
        let duty = state.duty()
            .keep_if(left_pressed && right_pressed);
        pwm.set_duty_off(Channel::C0, duty.unwrap_or(ZERO_DUTY));

        if state.is_brightness_dirty {
            let brightness = Brightness::custom(DISPLAY_PRECHARGE, state.config.brightness());
            display.set_brightness(brightness)
                .ignore();
        }
        if touched || left_pressed || right_pressed || (now - last_interaction) < SCREENSAVER_TIMEOUT {
            state.render_dirty(&mut display);
            if duty.is_none() {
                update_battery(&mut adc, &mut state, &mut blue, now);
            }
        } else if state.battery_charging {
            was_touched = screen_saver(&mut display, &mut rng, &mut adc, &mut touch, &mut left_btn, &mut right_btn, &mut green, now);
            now = timer.now();
            last_interaction = now;
            state.mark_all_dirty();
        } else {
            set_display(&mut state, &mut display, false);
        }
        timer.sleep_ms(IDLE_PERIOD as u32)
            .unwrap_or_else(|_| red.blink());
    }
}

fn handle_pressed(
    state: &mut State,
    adc: &mut Adc,
    left_pressed: bool,
    right_pressed: bool,
    now: Time,
) {
    match state.mode.clone() {
        Mode::Work { .. } if adc.is_usb_connected() => (),
        Mode::Work { duration, prev, cool_down, start, duty } => calc_work_progress_and_duty(state, adc, left_pressed, right_pressed, now, duration, prev, cool_down, start, duty),
        _ if state.buttons == (left_pressed, right_pressed) => (),
        _ if state.buttons.0 || state.buttons.1 => (),
        _ if left_pressed == right_pressed => (),
        Mode::Power if left_pressed => state.dec_power(),
        Mode::Power if right_pressed => state.inc_power(),
        Mode::Limit if left_pressed => state.dec_limit(),
        Mode::Limit if right_pressed => state.inc_limit(),
        Mode::Resistance if left_pressed => state.dec_resistance(),
        Mode::Resistance if right_pressed => state.inc_resistance(),
        Mode::Brightness if left_pressed => state.dec_brightness(),
        Mode::Brightness if right_pressed => state.inc_brightness(),
        _ => (),
    }
    state.set_pressed(left_pressed, right_pressed);
}

fn calc_work_progress_and_duty(
    state: &mut State,
    adc: &mut Adc,
    left_pressed: bool,
    right_pressed: bool,
    now: Time,
    mut duration: Time,
    prev: Time,
    cool_down: bool,
    start: Option<Time>,
    duty: Option<Duty>,
) {
    let rest_mv = match state.rest_mv {
        None => return,
        Some(mv) => mv,
    };
    let limit = state.limit_ms();
    let cool_down = duration >= limit || cool_down && (duration > 0 || left_pressed || right_pressed);
    match cool_down || !left_pressed || !right_pressed {
        true => duration -= min(now - prev, duration),
        _ if !state.buttons.0 => (),
        _ if !state.buttons.1 => (),
        _ => duration += now - prev,
    }
    state.set_work_duration(duration, now, cool_down);
    match start {
        _ if cool_down || duration == 0 && !left_pressed && !right_pressed => state.set_work_duty(None, None),
        None if duty.is_some() => (), // duty is measured
        None => {
            adc.start_measuring();
            state.set_work_duty(Some(now), Some(TEST_DUTY)); // todo MAX_DUTY
        }
        Some(start) if now - start < 100 => (), // measuring
        Some(_) => match adc.finish_measuring() {
            None => {
                state.set_work_duration(duration, now, true);
                state.set_work_duty(None, None);
            },
            Some(mv) => {
                state.set_load_mv(mv);
                let theoretical_max = state.config.milliwatts(VOLTS_MAX);
                let theoretical = state.config.milliwatts(rest_mv);
                let current = state.config.milliwatts(mv);
                let drawdown = (theoretical - current).max(0);
                let percents = state.config.power.percents() as MilliWatt;
                let mut target = (theoretical_max - drawdown) * percents / 100;
                target = min(target, current);
                let duty = (MAX_DUTY as MilliWatt * (target / 10) / (current / 10)) as Duty;
                state.set_work_duty(None, Some(duty * 0 + TEST_DUTY)); // todo duty
            }
        },
    }
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
    adc: &mut Adc,
    state: &mut State,
    blue: &mut PinOut<PushPull>,
    now: Time,
) {
    let connected = adc.is_usb_connected();
    let is_charging = is_charging() || connected; // todo remove '|| connected'
    state.set_usb_info(connected, is_charging);
    let need_update = state.battery_level.is_none() || adc.last_check == 0 || (now - adc.last_check) > BATTERY_PERIOD;
    let info = match () {
        _ if !state.is_progress_zero() => return,
        _ if connected => return state.reset_battery_info(),
        _ if !need_update => return,
        _ => adc.get_mv_and_level(now),
    };
    blue.blink();
    state.set_battery_info(info);
}

fn is_charging() -> bool {
    false // todo check charging leds
}

fn screen_saver(
    display: &mut Display,
    rng: &mut Rng,
    adc: &mut Adc,
    touch: &mut PinIn<Floating>,
    left_btn: &mut PinIn<PullUp>,
    right_btn: &mut PinIn<PullUp>,
    green: &mut PinOut<PushPull>,
    now: Time,
) -> bool {
    let mut flag = true;
    let mut start = true;
    let mut touched = false;
    let check_counter_max = 8;
    let mut check_counter = check_counter_max;
    while check_counter > 0 {
        match flag {
            true => green.on(),
            false => green.off(),
        }
        flag = !flag;
        draw_life(display, rng, false, start, now);
        start = false;
        check_counter -= 1;
        match () {
            _ if check_counter > 0 => (),
            _ if touch.is_high().unwrap_or(false) => touched = true,
            _ if left_btn.is_low().unwrap_or(false) => (),
            _ if right_btn.is_low().unwrap_or(false) => (),
            // todo remove '&& !connected'
            _ if !is_charging() && !adc.is_usb_connected() => (),
            _ => check_counter = check_counter_max,
        }
    }
    green.off();
    display.set_addr_mode(AddrMode::Horizontal)
        .ignore();
    return touched
}
