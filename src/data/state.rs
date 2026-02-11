use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::types::DeciSeconds;
use crate::values::{LIMIT_RANGE, RESISTANCE_RANGE};

pub struct State {
    pub mode: Mode,
    pub config: Config,

    pub buttons: (bool, bool),
    pub total: DeciSeconds,
    pub count: u32,
    pub watt: u8,

    pub battery_charging: bool,
    pub battery_level: Option<u8>,

    pub is_progress_dirty: bool,
    pub is_buttons_dirty: bool,
    pub is_header_dirty: bool,
    pub is_power_or_limit_dirty: bool,
    pub is_resistance_or_watt_dirty: bool,
    pub is_stat_dirty: bool,
    pub is_battery_dirty: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Work(0),
            config: Config::default(),

            buttons: (false, false),
            total: 7545,
            count: 1337,
            watt: 30,

            battery_charging: false,
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
}

impl State {

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
}