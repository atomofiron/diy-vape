use crate::core::charge_status::ChargeStatus;
use crate::data::battery::Battery;
use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::data::stats::Stats;
use crate::types::{Duty, MilliVolt, MilliWatt, Percent, Time};
use crate::values::{BRIGHTNESS_RANGE, LIMIT_RANGE, MW, RESISTANCE_RANGE, SECOND, VOLTS_MIN};

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,
    pub battery: Battery,

    pub buttons: (bool, bool), // left, right
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
            battery: Battery::default(),

            buttons: (false, false),

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

    pub fn set_load_mv(&mut self, mv: MilliVolt) {
        if self.battery.load != Some(mv) {
            self.battery.load = Some(mv);
            self.is_resistance_or_watts_dirty = true;
        }
    }

    pub fn set_charge_status(&mut self, charging: bool, full: bool) -> bool {
        let status = ChargeStatus::pick(charging, full);
        if status != self.battery.status {
            self.battery.status = status;
            self.is_statusbar_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn reset_battery_mv(&mut self) {
        self.set_battery_idle(None);
    }

    pub fn set_battery_idle(&mut self, mv: Option<MilliVolt>) {
        if mv != self.battery.idle {
            self.battery.idle = mv;
            self.is_statusbar_dirty = true;
            self.is_resistance_or_watts_dirty = true;
        }
    }

    pub fn watts(&self) -> Option<MilliWatt> {
        let mut mw = self.config.milliwatts(self.battery.load?);
        let percents = self.config.power.percents() as MilliWatt;
        mw = mw * percents / 100;
        let mut watts = mw / MW;
        if (mw % MW) >= (MW / 2) {
            watts += 1;
        }
        return Some(watts as MilliWatt)
    }

    pub fn set_pressed(&mut self, left: bool, right: bool) {
        if self.buttons != (left, right) {
            self.are_buttons_dirty = true;
            self.buttons.0 = left;
            self.buttons.1 = right;
        }
    }

    pub fn update_max_mv(&mut self) {
        let max = match self.battery.idle {
            Some(idle) => idle,
            None => return,
        };
        if max != self.config.battery_max {
            self.config.battery_max = max;
            self.is_statusbar_dirty = true;
        }
    }

    pub fn get_battery_level(&mut self) -> Option<Percent> {
        let max = self.config.battery_max;
        let now = self.battery.idle?;
        let percents = (now - VOLTS_MIN) * 100 / (max - VOLTS_MIN);
        return Some(percents.clamp(0, 100) as Percent)
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
