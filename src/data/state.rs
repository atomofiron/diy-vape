use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::data::stats::Stats;
use crate::types::{MilliVolt, Percent, Time};
use crate::values::{LIMIT_RANGE, MV, RESISTANCE_RANGE, SECOND};

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,

    pub buttons: (bool, bool), // left, right

    pub usb_connected: bool, // nrf52840
    pub battery_charging: bool, // 4056H
    pub battery_voltage: Option<MilliVolt>,
    pub battery_level: Option<Percent>,

    pub is_display_on: bool,

    pub is_header_dirty: bool,
    pub is_power_or_limit_dirty: bool,
    pub is_resistance_or_watt_dirty: bool,
    pub is_footer_dirty: bool,

    pub are_buttons_dirty: bool,
    pub is_progress_dirty: bool,
    pub is_stat_dirty: bool,
    pub is_battery_dirty: bool,
}

impl State {

    pub fn with(config: Config, stats: Stats) -> Self {
        Self {
            mode: Mode::default(),
            config,
            stats,

            buttons: (false, false),

            usb_connected: false,
            battery_charging: false,
            battery_voltage: None,
            battery_level: None,

            is_display_on: true,

            is_header_dirty: true,
            is_power_or_limit_dirty: true,
            is_resistance_or_watt_dirty: true,
            is_footer_dirty: true,

            are_buttons_dirty: true,
            is_progress_dirty: true,
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
            Mode::Work { .. } => (),
            Mode::Power | Mode::Limit => self.is_power_or_limit_dirty = true,
            Mode::Resistance => self.is_resistance_or_watt_dirty = true,
        }
    }

    pub fn set_work(&mut self, duration: Time, prev: Time, cool_down: bool) {
        match &mut self.mode {
            Mode::Work { duration: d, prev: p, cool_down: c } => {
                if *d != duration {
                    self.is_progress_dirty = true;
                }
                *d = duration;
                *p = prev;
                *c = cool_down;
            },
            _ => return,
        };
    }

    pub fn inc_power(&mut self) {
        self.config.power = self.config.power.inc();
        self.is_power_or_limit_dirty = true;
        self.is_header_dirty = true;
    }

    pub fn dec_power(&mut self) {
        self.config.power = self.config.power.dec();
        self.is_power_or_limit_dirty = true;
        self.is_header_dirty = true;
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

    pub fn volts(&self) -> Option<f32> {
        self.battery_voltage.map(|v| v as f32 / MV)
    }

    pub fn watts(&self) -> Option<u8> {
        Some(self.config.watts(self.volts()?))
    }

    pub fn set_pressed(&mut self, left: bool, right: bool) {
        if self.buttons != (left, right) {
            self.are_buttons_dirty = true;
            self.buttons.0 = left;
            self.buttons.1 = right;
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

    pub fn limit_ms(&self) -> Time {
        self.config.limit as Time * SECOND
    }
}

impl Default for State {

    fn default() -> Self {
        State::with(Config::default(), Stats::default())
    }
}
