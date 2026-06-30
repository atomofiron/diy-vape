use crate::core::charge_status::ChargeStatus;
use crate::core::ui::Ui;
use crate::data::action::Action;
use crate::data::battery::Battery;
use crate::data::buttons::Buttons;
use crate::data::config::Config;
use crate::data::edit_settings::EditSettings;
use crate::data::mode::Mode;
use crate::data::reset_puffs::ResetPuffs;
use crate::data::stats::Stats;
use crate::types::{DeciSecond, Duty, MilliVolt, Percent, Progress, Time};
use crate::values::{BRIGHTNESS_RANGE, DECI_SECOND, LIMIT_RANGE, PROGRESS_MAX, RESISTANCE_RANGE, SECOND, VOLTS_MIN};
use core::cmp::min;

pub struct State {
    pub mode: Mode,
    pub config: Config,
    pub stats: Stats,
    pub battery: Battery,
    pub buttons: Buttons,

    pub ui: Ui,

    pub touched: Option<Time>,
    pub last: Option<Action>,
    pub puff_duration: Time,
    pub puff_trigger: bool, // true = counted
}

impl State {

    pub fn with(config: Config, stats: Stats) -> Self {
        Self {
            mode: Mode::default(),
            config,
            stats,
            battery: Battery::default(),
            buttons: Buttons::default(),

            ui: Ui::default(),

            touched: None,
            last: None,
            puff_duration: 0,
            puff_trigger: false,
        }
    }

    pub fn buttons(&self, left: bool, right: bool) -> bool {
        self.buttons.left == left && self.buttons.right == right
    }
    
    pub fn touched(&self) -> bool {
        self.touched.is_some()
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
        self.mode = match &self.mode {
            Mode::Work { .. } => Mode::settings(),
            Mode::Settings(EditSettings::None) => Mode::Settings(EditSettings::Power),
            Mode::Settings(EditSettings::Power) => Mode::Settings(EditSettings::Limit),
            Mode::Settings(EditSettings::Limit) => Mode::Settings(EditSettings::Resistance),
            Mode::Settings(EditSettings::Resistance) => Mode::Settings(EditSettings::Brightness,),
            Mode::Settings(EditSettings::Brightness, ..) => Mode::settings(),
            Mode::Puffs(reset, ..) => {
                let mut reset = reset.clone();
                loop {
                    reset = reset.next();
                    match reset {
                        ResetPuffs::Coil if self.stats.coil > 0 => break,
                        ResetPuffs::Count if self.stats.count > 0 => break,
                        ResetPuffs::Total if self.stats.total > 0 => break,
                        ResetPuffs::None => break,
                        _ => continue,
                    }
                };
                Mode::Puffs(reset, None)
            },
            Mode::Battery => Mode::Battery,
        }
    }

    pub fn reset_mode(&mut self) {
        self.mode = Mode::default();
    }

    pub fn switch_tab(&mut self, next: bool) {
        self.mode = match &self.mode {
            Mode::Settings(..) => if next { Mode::puffs() } else { Mode::Battery },
            Mode::Puffs(..) => if next { Mode::Battery } else { Mode::settings() },
            Mode::Battery => if next { Mode::settings() } else { Mode::puffs() },
            Mode::Work { .. } => return,
        };
    }

    pub fn set_work_duration(
        &mut self,
        duration: Time,
        prev: Time,
        cool_down: bool,
    ) {
        match &mut self.mode {
            Mode::Work { duration: d, prev: p, cool_down: c, .. } => {
                *d = duration;
                *p = prev;
                *c = cool_down;
            }
            _ => (),
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
            _ => (),
        };
    }

    pub fn calc_puff_duration(&self) -> DeciSecond {
        let mut duration = (self.puff_duration / DECI_SECOND) as DeciSecond;
        let part = self.puff_duration % DECI_SECOND;
        if part >= DECI_SECOND / 2 {
            duration += 1
        };
        return duration
    }

    pub fn commit_puff_duration(&mut self) {
        self.stats.coil += self.calc_puff_duration();
        self.stats.total += self.calc_puff_duration();
        self.puff_duration = 0;
    }

    pub fn edit_power(&mut self, increment: bool) {
        let new = match increment {
            true => self.config.power.inc(),
            false => self.config.power.dec(),
        };
        if new != self.config.power {
            self.config.power = new;
            self.last = Some(Action::Power(true));
        }
    }

    pub fn edit_limit(&mut self, increment: bool) {
        let new = match self.config.limit {
            limit if increment && limit < LIMIT_RANGE.end => limit + 1,
            limit if !increment && limit > LIMIT_RANGE.start => limit - 1,
            _ => return,
        };
        if new != self.config.limit {
            self.config.limit = new;
            self.last = Some(Action::Limit(increment));
        }
    }

    pub fn edit_resistance(&mut self, increment: bool) {
        let new = match self.config.resistance {
            resistance if increment && resistance < RESISTANCE_RANGE.end => resistance + 1,
            resistance if !increment && resistance > RESISTANCE_RANGE.start => resistance - 1,
            _ => return,
        };
        if new != self.config.resistance {
            self.config.resistance = new;
            self.last = Some(Action::Resistance(increment));
        }
    }

    pub fn edit_brightness(&mut self, increment: bool) {
        let new = match self.config.brightness {
            brightness if increment && brightness < BRIGHTNESS_RANGE.end => brightness + 1,
            brightness if !increment && brightness > BRIGHTNESS_RANGE.start => brightness - 1,
            _ => return,
        };
        if new != self.config.brightness {
            self.config.brightness = new;
            self.last = Some(Action::Brightness(increment));
        }
    }

    pub fn set_load_mv(&mut self, mv: MilliVolt) {
        self.battery.load = Some(mv);
    }

    pub fn set_charge_status(&mut self, charging: bool, full: bool, reverse: bool) -> bool {
        let status = ChargeStatus::pick(charging, full, reverse);
        if status != self.battery.status {
            self.battery.status = status;
            true
        } else {
            false
        }
    }

    pub fn reset_battery_mv(&mut self) {
        self.set_battery_idle(None);
    }

    pub fn set_battery_idle(&mut self, mv: Option<MilliVolt>) {
        self.battery.idle = mv;
    }

    pub fn set_pressed(&mut self, left: bool, right: bool) {
        self.buttons.left = left;
        self.buttons.right = right;
    }

    pub fn update_full_mv(&mut self) {
        let full = match self.battery.idle {
            Some(idle) => idle,
            None => return,
        };
        self.config.battery_full = full;
    }

    pub fn get_battery_level(&mut self) -> Option<Percent> {
        let full = self.config.battery_full;
        let now = self.battery.idle?;
        let percents = (now - VOLTS_MIN) as u32 * 100 / (full - VOLTS_MIN) as u32;
        return Some(percents.clamp(0, 100) as Percent)
    }

    pub fn limit_ms(&self) -> Time {
        self.config.limit as Time * SECOND
    }

    pub fn progress(&self, duration: Time) -> Progress {
        let limit = self.limit_ms();
        let max = PROGRESS_MAX as Time;
        return min(duration * max / limit, max) as Progress
    }
}
