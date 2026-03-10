use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::data::stats::Stats;
use crate::types::{MilliVolts, Percents};
use crate::util::round;
use crate::values::{LIMIT_RANGE, PROGRESS_MAX, RESISTANCE_RANGE};

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,

    pub buttons: (bool, bool), // left, right

    pub usb_connected: bool, // nrf52840
    pub battery_charging: bool, // 4056H
    pub battery_voltage: Option<MilliVolts>,
    pub battery_level: Option<Percents>,

    pub is_display_on: bool,

    pub are_buttons_dirty: bool,
    pub is_progress_dirty: bool,
    pub is_header_dirty: bool,
    pub is_power_or_limit_dirty: bool,
    pub is_resistance_or_watt_dirty: bool,
    pub is_footer_dirty: bool,
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

            is_display_on: true,

            are_buttons_dirty: true,
            is_progress_dirty: true,
            is_header_dirty: true,
            is_power_or_limit_dirty: true,
            is_resistance_or_watt_dirty: true,
            is_footer_dirty: true,
            is_stat_dirty: true,
            is_battery_dirty: true,
        }
    }

    pub fn next_mode(&mut self) {
        self.is_header_dirty = true;
        self.mark_current_dirty();
        self.mode = self.mode.next();
        self.mark_current_dirty();
    }

    fn mark_current_dirty(&mut self) {
        match self.mode {
            Mode::Work(_) => (),
            Mode::Power | Mode::Limit => self.is_power_or_limit_dirty = true,
            Mode::Resistance => self.is_resistance_or_watt_dirty = true,
        }
    }

    pub fn inc_progress(&mut self) {
        match &mut self.mode {
            Mode::Work(PROGRESS_MAX) => return,
            Mode::Work(p) => *p += 3,
            _ => return,
        }
        self.is_progress_dirty = true;
    }

    pub fn reset_progress(&mut self) {
        match &mut self.mode {
            Mode::Work(0) => return,
            Mode::Work(p) => *p = 0,
            _ => return,
        }
        self.is_progress_dirty = true;
    }

    pub fn inc_power(&mut self) {
        self.config.power = self.config.power.inc();
        self.is_power_or_limit_dirty = true;
    }

    pub fn dec_power(&mut self) {
        self.config.power = self.config.power.dec();
        self.is_power_or_limit_dirty = true;
    }

    pub fn inc_limit(&mut self) {
        if self.config.limit < LIMIT_RANGE.end {
            self.config.limit += 1;
            self.is_power_or_limit_dirty = true;
        }
    }

    pub fn dec_limit(&mut self) {
        if self.config.limit > LIMIT_RANGE.start {
            self.config.limit -= 1;
            self.is_power_or_limit_dirty = true;
        }
    }

    pub fn inc_resistance(&mut self) {
        if self.config.resistance < RESISTANCE_RANGE.end {
            self.config.resistance += 1;
            self.is_resistance_or_watt_dirty = true;
        }
    }

    pub fn dec_resistance(&mut self) {
        if self.config.resistance > RESISTANCE_RANGE.start {
            self.config.resistance -= 1;
            self.is_resistance_or_watt_dirty = true;
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

    pub fn set_pressed(&mut self, left: bool, right: bool) -> bool {
        return if self.buttons != (left, right) {
            self.are_buttons_dirty = true;
            self.buttons.0 = left;
            self.buttons.1 = right;
            true
        } else {
            false
        }
    }

    pub fn is_smth_dirty(&self) -> bool {
        return self.are_buttons_dirty
            || self.is_progress_dirty
            || self.is_header_dirty
            || self.is_power_or_limit_dirty
            || self.is_resistance_or_watt_dirty
            || self.is_footer_dirty
            || self.is_stat_dirty
            || self.is_battery_dirty
    }

    pub fn mark_all_dirty(&mut self) {
        self.is_header_dirty = true;
        self.is_power_or_limit_dirty = true;
        self.is_resistance_or_watt_dirty = true;
        self.is_footer_dirty = true;
    }
}

impl Default for State {

    fn default() -> Self {
        State::with(Config::default(), Stats::default())
    }
}
