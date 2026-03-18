use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::data::stats::Stats;
use crate::types::{Duty, MilliVolt, MilliWatt, Percent, Time};
use crate::values::{BRIGHTNESS_RANGE, LIMIT_RANGE, MW, RESISTANCE_RANGE, SECOND};

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,

    pub buttons: (bool, bool), // left, right

    pub battery_charging: bool, // 4056H
    pub battery_charging_stdby: bool, // todo remove
    pub battery_level: Option<Percent>,
    pub rest_mv: Option<MilliVolt>,
    pub load_mv: Option<MilliVolt>,

    pub is_display_on: bool,

    pub is_header_dirty: bool,
    pub is_power_or_limit_dirty: bool,
    pub is_resistance_or_watts_dirty: bool,
    pub is_brightness_dirty: bool,
    pub is_footer_dirty: bool,

    pub are_buttons_dirty: bool,
    pub is_statusbar_dirty: bool,
}

impl State {

    pub fn with(config: Config, stats: Stats) -> Self {
        Self {
            mode: Mode::default(),
            config,
            stats,

            buttons: (false, false),

            battery_charging: false,
            battery_charging_stdby: false,
            battery_level: None,
            rest_mv: None,
            load_mv: None,

            is_display_on: true,

            is_header_dirty: true,
            is_power_or_limit_dirty: true,
            is_resistance_or_watts_dirty: true,
            is_brightness_dirty: true,
            is_footer_dirty: true,

            are_buttons_dirty: true,
            is_statusbar_dirty: true,
        }
    }

    pub fn is_work(&self) -> bool {
        match self.mode {
            Mode::Work { .. } => true,
            _ => false,
        }
    }

    pub fn duty(&self) -> Option<Duty> {
        match self.mode {
            Mode::Work { duty, .. } => duty,
            _ => None,
        }
    }

    pub fn is_progress_zero(&self) -> bool {
        match self.mode {
            Mode::Work { duration, .. } => duration == 0,
            _ => true,
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
            Mode::Work { .. } => self.is_statusbar_dirty = true,
            Mode::Power | Mode::Limit => self.is_power_or_limit_dirty = true,
            Mode::Resistance => self.is_resistance_or_watts_dirty = true,
            Mode::Brightness => self.is_brightness_dirty = true,
        }
    }

    pub fn set_work_duration(
        &mut self,
        duration: Time,
        prev: Time,
        cool_down: bool,
    ) {
        match &mut self.mode {
            Mode::Work { duration: d, prev: p, cool_down: c, .. } => {
                if *d != duration {
                    self.is_header_dirty = true;
                }
                *d = duration;
                *p = prev;
                *c = cool_down;
            }
            _ => return,
        };
    }

    pub fn set_work_duty(
        &mut self,
        start: Option<Time>,
        duty: Option<Duty>,
    ) {
        match &mut self.mode {
            Mode::Work { start: s, duty: d, .. } => {
                *s = start;
                *d = duty;
            }
            _ => return,
        };
    }

    pub fn inc_power(&mut self) {
        self.config.power = self.config.power.inc();
        self.is_power_or_limit_dirty = true;
        self.is_resistance_or_watts_dirty = true;
        self.is_header_dirty = true;
    }

    pub fn dec_power(&mut self) {
        self.config.power = self.config.power.dec();
        self.is_power_or_limit_dirty = true;
        self.is_resistance_or_watts_dirty = true;
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
            self.is_resistance_or_watts_dirty = true;
        }
    }

    pub fn dec_resistance(&mut self) {
        if self.config.resistance > RESISTANCE_RANGE.start {
            self.config.resistance -= 1;
            self.is_resistance_or_watts_dirty = true;
        }
    }

    pub fn inc_brightness(&mut self) {
        if self.config.brightness < BRIGHTNESS_RANGE.end {
            self.config.brightness += 1;
            self.is_brightness_dirty = true;
        }
    }

    pub fn dec_brightness(&mut self) {
        if self.config.brightness > BRIGHTNESS_RANGE.start {
            self.config.brightness -= 1;
            self.is_brightness_dirty = true;
        }
    }

    pub fn set_load_mv(&mut self, mw: MilliVolt) {
        if self.load_mv != Some(mw) {
            self.load_mv = Some(mw);
            self.is_resistance_or_watts_dirty = true;
        }
    }

    pub fn set_charging_info(&mut self, charging: bool, stdby: bool) {
        if charging != self.battery_charging || stdby != self.battery_charging_stdby {
            self.battery_charging = charging;
            self.battery_charging_stdby = stdby;
            self.is_statusbar_dirty = true;
        }
    }

    pub fn reset_battery_info(&mut self) {
        if self.rest_mv.is_some() || self.battery_level.is_some() {
            self.set_battery_info(None);
        }
    }

    pub fn set_battery_info(&mut self, new: Option<(MilliVolt, Percent)>) {
        self.rest_mv = new.map(|(mv, _)| mv);
        self.battery_level = new.map(|(_, level)| level);
        self.is_statusbar_dirty = true;
        self.is_resistance_or_watts_dirty = true;
    }

    pub fn watts(&self) -> Option<u8> {
        let mut mw = self.config.milliwatts(self.load_mv?);
        let percents = self.config.power.percents() as MilliWatt;
        mw = mw * percents / 100;
        let mut watts = mw / MW;
        if (mw % MW) >= (MW / 2) {
            watts += 1;
        }
        Some(watts as u8)
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
            || self.is_header_dirty
            || self.is_power_or_limit_dirty
            || self.is_resistance_or_watts_dirty
            || self.is_brightness_dirty
            || self.is_statusbar_dirty
    }

    pub fn mark_all_dirty(&mut self) {
        self.is_header_dirty = true;
        self.is_power_or_limit_dirty = true;
        self.is_resistance_or_watts_dirty = true;
        match self.mode {
            Mode::Work { .. } => self.is_statusbar_dirty = true,
            _ => self.is_brightness_dirty = true,
        }
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
