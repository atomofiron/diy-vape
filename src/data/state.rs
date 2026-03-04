use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::data::stats::Stats;
use crate::types::{MilliVolts, Percents};
use crate::util::round;
use crate::values::{LIMIT_RANGE, RESISTANCE_RANGE};

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,

    pub buttons: (bool, bool),

    pub usb_connected: bool, // nrf52840
    pub battery_charging: bool, // 4056H
    pub battery_voltage: Option<MilliVolts>,
    pub battery_level: Option<Percents>,

    pub is_progress_dirty: bool,
    pub is_buttons_dirty: bool,
    pub is_header_dirty: bool,
    pub is_power_or_limit_dirty: bool,
    pub is_resistance_or_watt_dirty: bool,
    pub is_stat_dirty: bool,
    pub is_battery_dirty: bool,
}

impl State {

    pub fn with(config: Config, stats: Stats) -> Self {
        Self {
            mode: Mode::Work(0),
            config,
            stats,

            buttons: (false, false),

            usb_connected: false,
            battery_charging: false,
            battery_voltage: None,
            battery_level: None,

            is_progress_dirty: true,
            is_buttons_dirty: true,
            is_header_dirty: true,
            is_power_or_limit_dirty: true,
            is_resistance_or_watt_dirty: true,
            is_stat_dirty: true,
            is_battery_dirty: true,
        }
    }

    pub fn next_mode(&mut self) {
        self.mode = self.mode.next()
    }

    pub fn inc_progress(&mut self) {
        match &mut self.mode {
            Mode::Work(255) => (),
            Mode::Work(p) => *p += 3,
            _ => (),
        }
    }

    pub fn reset_progress(&mut self) {
        if let Mode::Work(p) = &mut self.mode {
            *p = 0;
        }
    }

    pub fn inc_power(mut self) {
        self.config.power = self.config.power.inc()
    }

    pub fn dec_power(mut self) {
        self.config.power = self.config.power.dec()
    }

    pub fn inc_limit(mut self) {
        if self.config.limit < LIMIT_RANGE.end {
            self.config.limit += 1
        }
    }

    pub fn dec_limit(mut self) {
        if self.config.limit > LIMIT_RANGE.start {
            self.config.limit -= 1
        }
    }

    pub fn inc_resistance(mut self) {
        if self.config.resistance < RESISTANCE_RANGE.end {
            self.config.resistance += 1
        }
    }

    pub fn dec_resistance(mut self) {
        if self.config.resistance > RESISTANCE_RANGE.start {
            self.config.resistance -= 1
        }
    }

    pub fn watts(&self) -> Option<u8> {
        let volts = self.battery_voltage? as f32 / 1000f32;
        let ohms = self.config.resistance as f32 / 10f32;
        return match round(volts * volts / ohms) {
            w if w < 0 => None,
            w if w > 255 => None,
            w => Some(w as u8),
        }
    }
}

impl Default for State {

    fn default() -> Self {
        State::with(Config::default(), Stats::default())
    }
}
