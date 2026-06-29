#![no_std]
#![no_main]

use core::cmp::min;
use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::DrawTarget;
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
use vape::core::charging::Charging;
use vape::core::renderer::Renderer;
use vape::core::timer::Timer;
use vape::data::action::Action;
use vape::data::config::Config;
use vape::data::edit_settings::EditSettings;
use vape::data::mode::Mode;
use vape::data::reset_puffs::ResetPuffs;
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
use vape::values::{BATTERY_PERIOD, DISPLAY_PRECHARGE, IDLE_PERIOD, PUFF_THRESHOLD, SCREENSAVER_TIMEOUT, SECOND, SLEEP_PERIOD, VOLTS_FULL};

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

    /*peripherals.POWER.dcdcen.write(|w| w.dcdcen().disabled());
    peripherals.POWER.vbusdetect.write(|w| w.vbusdetect().disabled());*/

    let port0 = Parts0::new(peripherals.P0);
    let port1 = Parts1::new(peripherals.P1);

    let mut touch = port1.p1_13.into_floating_input().degrade();
    let mut left_btn = port1.p1_12.into_pullup_input().degrade();
    let mut right_btn = port1.p1_11.into_pullup_input().degrade();

    let mut red = port0.p0_26.into_push_pull_output(Level::High).degrade();
    let mut green = port0.p0_30.into_push_pull_output(Level::High).degrade();
    let mut blue = port0.p0_06.into_push_pull_output(Level::High).degrade();

    blue.blink();

    let load = port1.p1_15.into_push_pull_output(Level::Low).degrade();
    let pwm = Pwm::new(peripherals.PWM0);
    pwm.set_output_pin(Channel::C0, load);
    pwm.set_prescaler(Prescaler::Div1);
    pwm.set_max_duty(MAX_DUTY);
    pwm.enable();

    let mut timer = Timer::init(peripherals.RTC1, peripherals.CLOCK)
        .unwrap();

    let mut charging = Charging::new(
        port0.p0_02.into_floating_input().degrade(),
        port0.p0_03.into_floating_input().degrade(),
        port0.p0_28.into_floating_input().degrade(),
    );
    let scl = port0.p0_05.into_floating_input()
        .degrade();
    let sda = port0.p0_04.into_floating_input()
        .degrade();

    let pins = TwimPins { scl, sda };
    let i2c = Twim::new(peripherals.TWIM0, pins, Frequency::K400);
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    match display.init() {
        Ok(_) => display.clear(BinaryColor::Off)
            .and_then(|_| display.flush())
            .ignore(),
        Err(_) => red.on(), // DisplayError is unaccessible >:(
    }

    let flash = AsyncFlash::from(peripherals.NVMC);
    let mut storage = flash.storage();

    let mut config_buf = [0u8; Config::FLASH_BUFFER_SIZE];
    let mut config = storage.read::<Config>(&mut config_buf)
        .await.soft_unwrap()
        .flat()
        .unwrap_or_default();

    let mut stats_buf = [0u8; Stats::FLASH_BUFFER_SIZE];
    let mut stats = storage.read::<Stats>(&mut stats_buf)
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
    let mut state = State::with(config.clone(), stats.clone());
    apply_brightness(&state, &mut display);

    green.blink();

    let mut is_display_on = true;
    let mut last_interaction = timer.now();
    loop {
        let touched = touch.is_high().unwrap_or(false);
        let left_pressed = left_btn.is_low().unwrap_or(false);
        let right_pressed = right_btn.is_low().unwrap_or(false);
        if !state.touched() && touched {
            green.on();
            if is_display_on && !left_pressed && !right_pressed {
                state.next_mode();
            }
        } else if state.touched() && !touched {
            green.off()
        }
        let mut interaction = state.touched() != touched;
        interaction = interaction || !state.buttons(left_pressed, right_pressed);
        interaction = update_charging(&mut state, &mut charging) || interaction;
        interaction = adc.update_usb_connection() || interaction;
        let now = timer.now();
        if interaction {
            last_interaction = now;
        }
        match state.touched {
            None if touched => state.touched = Some(now),
            Some(_) if !touched => state.touched = None,
            Some(time) if now > (time + SECOND * 2) => {
                state.reset_mode();
                screen_saver(&mut state, &mut display, &mut charging, &mut rng, &mut adc, &mut timer, &mut touch, &mut left_btn, &mut right_btn, &mut green, now);
                state.force_render(&mut display);
                continue
            },
            Some(time) if !state.mode.is_work() && now > (time + SECOND) => state.reset_mode(),
            _ => (),
        }
        if !is_display_on {
            match interaction {
                true => {
                    is_display_on = true;
                    display.set_display_on(true)
                        .ignore()
                },
                false => timer.sleep_ms(SLEEP_PERIOD as u32)
                    .unwrap_or_else(|_| red.blink()),
            }
            continue
        }

        handle_pressed(&mut state, &mut adc, left_pressed, right_pressed, now);
        let duty = state.duty()
            .keep_if(left_pressed && right_pressed);
        pwm.set_duty_off(Channel::C0, duty.unwrap_or(ZERO_DUTY));

        if let Some(Action::Brightness(..)) = state.last {
            apply_brightness(&state, &mut display);
        }
        if touched || left_pressed || right_pressed || duty.is_some() || (now - last_interaction) < SCREENSAVER_TIMEOUT {
            state.render(&mut display);
            if duty.is_none() {
                update_battery(&mut state, &mut adc, &mut blue, now);
                if state.battery.is_full() {
                    state.update_full_mv();
                }
            }
        } else if state.battery.status.is_powered() {
            state.reset_mode();
            screen_saver(&mut state, &mut display, &mut charging, &mut rng, &mut adc, &mut timer, &mut touch, &mut left_btn, &mut right_btn, &mut green, now);
            update_battery(&mut state, &mut adc, &mut blue, now);
            state.force_render(&mut display);
        } else if is_display_on {
            state.reset_mode();
            is_display_on = false;
            display.set_display_on(false)
                .ignore();
        }
        if state.config != config {
            config = state.config.clone();
            storage.save(&mut config_buf, config.clone())
                .await.soft_unwrap();
        }
        if state.stats != stats {
            stats = state.stats.clone();
            storage.save(&mut stats_buf, stats.clone())
                .await.soft_unwrap();
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
    if !state.mode.is_work() && (state.buttons.left ^ state.buttons.right) && left_pressed && right_pressed {
        revert_last(state);
        state.reset_mode();
    }
    if left_pressed == right_pressed {
        state.last = None
    }
    match state.mode {
        Mode::Work { duty, .. } if duty.is_none() && adc.fetch_usb_connection() => (),
        Mode::Work { duration, prev, cool_down, start, duty } => calc_work_progress_and_duty_and_stats(state, adc, left_pressed, right_pressed, now, duration, prev, cool_down, start, duty),
        _ if state.buttons(left_pressed, right_pressed) => (),
        _ if state.buttons.left || state.buttons.right => (),
        _ if left_pressed == right_pressed => (),
        Mode::Settings(EditSettings::None) |
        Mode::Puffs(ResetPuffs::None) |
        Mode::Battery => state.switch_tab(right_pressed),
        Mode::Settings(EditSettings::Power) => state.edit_power(right_pressed),
        Mode::Settings(EditSettings::Limit) => state.edit_limit(right_pressed),
        Mode::Settings(EditSettings::Resistance) => state.edit_resistance(right_pressed),
        Mode::Settings(EditSettings::Brightness) => state.edit_brightness(right_pressed),
        _ => (),
    }
    state.set_pressed(left_pressed, right_pressed);
}

fn revert_last(state: &mut State) {
    let last = match &state.last {
        Some(last) => last,
        None => return,
    };
    match last {
        Action::Power(increment) => state.edit_power(!increment),
        Action::Limit(increment) => state.edit_limit(!increment),
        Action::Resistance(increment) => state.edit_resistance(!increment),
        Action::Brightness(increment) => state.edit_brightness(!increment),
    }
    state.last = None;
}

fn calc_work_progress_and_duty_and_stats(
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
    let idle_mv = match state.battery.idle {
        None => return,
        Some(mv) => mv,
    };
    let limit = state.limit_ms();
    let cool_down = duration >= limit || cool_down && (duration > 0 || left_pressed || right_pressed);
    match cool_down || !left_pressed || !right_pressed {
        true => duration -= min(now - prev, duration),
        _ if !state.buttons.left => (),
        _ if !state.buttons.right => (),
        _ => {
            let dt = now - prev;
            duration += dt;
            state.puff_duration += dt;
        },
    }
    commit_stats(state, left_pressed, right_pressed, duration);
    state.set_work_duration(duration, now, cool_down);
    match start {
        _ if cool_down || duration == 0 && !left_pressed && !right_pressed => state.set_work_duty(None, None),
        None if duty.is_some() => (), // duty is measured
        _ if !left_pressed || !right_pressed => if start.is_some() {
            adc.stop_measuring()
                .soft_unwrap();
            state.set_work_duty(None, duty);
        },
        None => {
            adc.start_measuring()
                .soft_unwrap();
            state.set_work_duty(Some(now), Some(TEST_DUTY)); // todo MAX_DUTY
        }
        Some(start) if now - start < 100 => (), // measuring
        Some(_) => match adc.finish_measuring() {
            Err(_) => {
                state.set_work_duration(duration, now, true);
                state.set_work_duty(None, None);
            },
            Ok(mv) => {
                state.set_load_mv(mv);
                let theoretical_max = state.config.milliwatts(VOLTS_FULL);
                let theoretical = state.config.milliwatts(idle_mv);
                let current = state.config.milliwatts(mv);
                let drawdown = (theoretical - current).max(0);
                let percents = state.config.power.percents() as MilliWatt;
                let mut target = (theoretical_max - drawdown) * percents / 100;
                target = min(target, current);
                let duty = (MAX_DUTY as u32 * target / current as u32) as Duty;
                state.set_work_duty(None, Some(duty * 0 + TEST_DUTY)); // todo duty
            }
        },
    }
}

fn commit_stats(state: &mut State, left_pressed: bool, right_pressed: bool, duration: Time) {
    if state.buttons.left && state.buttons.right && (!left_pressed || !right_pressed) {
        state.commit_puff_duration();
    }
    if !state.puff_trigger && duration > PUFF_THRESHOLD {
        state.stats.count += 1;
        state.puff_trigger = true;
    } else if state.puff_trigger && duration == 0 {
        state.puff_trigger = false;
    }
}

fn apply_brightness(state: &State, display: &mut Display) {
    let brightness = Brightness::custom(DISPLAY_PRECHARGE, state.config.brightness());
    display.set_brightness(brightness)
        .ignore();
}

fn update_battery(
    state: &mut State,
    adc: &mut Adc,
    blue: &mut PinOut<PushPull>,
    now: Time,
) {
    let need_update = state.battery.idle.is_none() || adc.last_check == 0 || (now - adc.last_check) > BATTERY_PERIOD;
    let mv = match () {
        _ if !state.is_progress_zero() => return,
        _ if adc.fetch_usb_connection() => return state.reset_battery_mv(),
        _ if !need_update => return,
        _ => adc.measure(now)
            .soft_unwrap()
            .flat(),
    };
    blue.blink();
    state.set_battery_idle(mv);
}

fn update_charging(
    state: &mut State,
    charging: &mut Charging,
) -> bool {
    let is_charging = charging.is_charging()
        .soft_unwrap_or(false);
    let is_full = charging.is_full()
        .soft_unwrap_or(false);
    let is_reverse = charging.is_reverse()
        .soft_unwrap_or(false);
    return state.set_charge_status(is_charging, is_full, is_reverse)
}

fn screen_saver(
    state: &mut State,
    display: &mut Display,
    charging: &mut Charging,
    rng: &mut Rng,
    adc: &mut Adc,
    timer: &mut Timer,
    touch: &mut PinIn<Floating>,
    left_btn: &mut PinIn<PullUp>,
    right_btn: &mut PinIn<PullUp>,
    green: &mut PinOut<PushPull>,
    now: Time,
) {
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
            _ if update_charging(state, charging) => (),
            _ if adc.update_usb_connection() => (),
            _ => check_counter = check_counter_max,
        }
    }
    green.off();
    display.set_addr_mode(AddrMode::Horizontal)
        .ignore();
    state.touched = match touched {
        true => Some(timer.now()),
        false => None,
    };
}
