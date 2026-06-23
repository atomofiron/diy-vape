use crate::core::charge_status::ChargeStatus;
use crate::data::action::Action;
use crate::data::battery::Battery;
use crate::data::buttons::Buttons;
use crate::data::config::Config;
use crate::data::mode::Mode;
use crate::data::stats::Stats;
use crate::data::tab::Tab;
use crate::types::{DeciSecond, Duty, MilliVolt, MilliWatt, Percent, Time};
use crate::values::{BRIGHTNESS_RANGE, DECI_SECOND, LIMIT_RANGE, MW, RESISTANCE_RANGE, SECOND, VOLTS_MIN};

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,
    pub battery: Battery,

    pub buttons: Buttons,
    pub is_display_on: bool,
    pub last: Option<Action>,
    pub puff_duration: Time,
    pub puff_trigger: bool, // true = counted

    pub are_buttons_dirty: bool,
    pub is_header_dirty: bool,
    pub is_power_or_limit_dirty: bool,
    pub is_resistance_or_watts_dirty: bool,
    pub is_statusbar_dirty: bool,
    pub is_status_dirty: bool,
    pub is_brightness_dirty: bool,
}

impl State {

    pub fn with(config: Config, stats: Stats) -> Self {
        Self {
            mode: Mode::default(),
            config,
            stats,
            battery: Battery::default(),

            buttons: Buttons::default(),
            is_display_on: true,
            last: None,
            puff_duration: 0,
            puff_trigger: false,

            are_buttons_dirty: true,
            is_header_dirty: true,
            is_power_or_limit_dirty: true,
            is_resistance_or_watts_dirty: true,
            is_statusbar_dirty: true,
            is_status_dirty: false,
            is_brightness_dirty: true,
        }
    }

    pub fn buttons(&self, left: bool, right: bool) -> bool {
        self.buttons.left == left && self.buttons.right == right
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
        self.set_mode(self.mode.next());
    }

    pub fn reset_mode(&mut self) {
        self.set_mode(Mode::default());
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.is_header_dirty = true;
        self.mark_current_dirty();
        self.mode = mode;
        self.is_statusbar_dirty = self.mode.is_work();
        self.is_brightness_dirty = self.mode.is_settings();
        self.mark_current_dirty();
    }

    pub fn next_tab(&mut self) {
        let new = match self.mode {
            Mode::Tabs(Tab::Settings) => Tab::Puffs,
            Mode::Tabs(Tab::Puffs) => Tab::Settings,
            _ => return,
        };
        self.mode = Mode::Tabs(new);
        self.is_header_dirty = true;
    }

    fn mark_current_dirty(&mut self) {
        match self.mode {
            Mode::Work { .. } => self.is_statusbar_dirty = true,
            Mode::Tabs(..) => (),
            Mode::Power | Mode::Limit => self.is_power_or_limit_dirty = true,
            Mode::Resistance => self.is_resistance_or_watts_dirty = true,
            Mode::Brightness => self.is_brightness_dirty = true,
            Mode::ResetCoil => (), // todo
            Mode::ResetStats => (), // todo
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

    pub fn calc_stat_total(&mut self) -> DeciSecond {
        let mut duration = (self.puff_duration / DECI_SECOND) as DeciSecond;
        let part = self.puff_duration & DECI_SECOND;
        if part >= DECI_SECOND / 2 {
            duration += 1
        };
        return self.stats.total + duration
    }

    pub fn commit_stat_total(&mut self) {
        self.stats.total = self.calc_stat_total();
        self.puff_duration = 0;
    }

    pub fn inc_power(&mut self) {
        let new = self.config.power.inc();
        if new != self.config.power {
            self.config.power = new;
            self.is_power_or_limit_dirty = true;
            self.is_resistance_or_watts_dirty = true;
            self.is_header_dirty = true;
            self.last = Some(Action::Power(true));
        }
    }

    pub fn dec_power(&mut self) {
        let new = self.config.power.dec();
        if new != self.config.power {
            self.config.power = new;
            self.is_power_or_limit_dirty = true;
            self.is_resistance_or_watts_dirty = true;
            self.is_header_dirty = true;
            self.last = Some(Action::Power(false));
        }
    }

    pub fn inc_limit(&mut self) {
        if self.config.limit < LIMIT_RANGE.end {
            self.config.limit += 1;
            self.is_power_or_limit_dirty = true;
            self.last = Some(Action::Limit(true));
        }
    }

    pub fn dec_limit(&mut self) {
        if self.config.limit > LIMIT_RANGE.start {
            self.config.limit -= 1;
            self.is_power_or_limit_dirty = true;
            self.last = Some(Action::Limit(false));
        }
    }

    pub fn inc_resistance(&mut self) {
        if self.config.resistance < RESISTANCE_RANGE.end {
            self.config.resistance += 1;
            self.is_resistance_or_watts_dirty = true;
            self.last = Some(Action::Resistance(true));
        }
    }

    pub fn dec_resistance(&mut self) {
        if self.config.resistance > RESISTANCE_RANGE.start {
            self.config.resistance -= 1;
            self.is_resistance_or_watts_dirty = true;
            self.last = Some(Action::Resistance(false));
        }
    }

    pub fn inc_brightness(&mut self) {
        if self.config.brightness < BRIGHTNESS_RANGE.end {
            self.config.brightness += 1;
            self.is_brightness_dirty = true;
            self.last = Some(Action::Brightness(true));
        }
    }

    pub fn dec_brightness(&mut self) {
        if self.config.brightness > BRIGHTNESS_RANGE.start {
            self.config.brightness -= 1;
            self.is_brightness_dirty = true;
            self.last = Some(Action::Brightness(false));
        }
    }

    pub fn set_load_mv(&mut self, mv: MilliVolt) {
        if self.battery.load != Some(mv) {
            self.battery.load = Some(mv);
            self.is_resistance_or_watts_dirty = true;
        }
    }

    pub fn set_charge_status(&mut self, charging: bool, full: bool, reverse: bool) -> bool {
        let status = ChargeStatus::pick(charging, full, reverse);
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
        if !self.buttons(left, right) {
            self.are_buttons_dirty = true;
            self.buttons.left = left;
            self.buttons.right = right;
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
        let percents = (now - VOLTS_MIN) as u32 * 100 / (max - VOLTS_MIN) as u32;
        return Some(percents.clamp(0, 100) as Percent)
    }

    pub fn is_smth_dirty(&self) -> bool {
        self.are_buttons_dirty
            || self.is_header_dirty
            || self.is_power_or_limit_dirty
            || self.is_resistance_or_watts_dirty
            || self.is_status_dirty
            || self.is_brightness_dirty
            || self.is_statusbar_dirty
    }

    pub fn mark_all_dirty(&mut self) {
        self.are_buttons_dirty = true;
        self.is_header_dirty = true;
        self.is_power_or_limit_dirty = true;
        self.is_resistance_or_watts_dirty = true;
        self.is_status_dirty = true;
        self.is_statusbar_dirty = true;
        self.is_brightness_dirty = true;
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
