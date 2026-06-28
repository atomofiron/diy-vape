use crate::core::strings::{BRIGHTNESS, LIMIT, POWER_100, POWER_25, POWER_50, POWER_75, RESET_ALL, RESET_COIL, RESISTANCE};
use crate::core::ui::header::Header;
use crate::core::ui::power_and_limit;
use crate::core::ui::tab::Tab;
use crate::core::ui::widget::Widget;
use crate::core::ui::Ui;
use crate::data::edit_settings::EditSettings;
use crate::data::mode::Mode;
use crate::data::power::Power;
use crate::data::reset_puffs::ResetPuffs;
use crate::data::state::State;
use crate::types::Display;

pub trait Renderer {
    fn render(&mut self, display: &mut Display);
}

trait RendererImpl {
    fn settings_header(&self, edit: EditSettings) -> Header;
    fn render_power(&self) -> Widget;
    fn render_resistance(&self) -> Widget;
    fn render_statusbar(&self) -> Widget;
    fn render_brightness(&self) -> Widget;
}

impl Renderer for State {

    fn render(&mut self, display: &mut Display) {
        let new = match self.mode.clone() {
            Mode::Work { duration, .. } => Ui {
                buttons: self.buttons.clone(),
                header: Header::Progress(self.progress(duration)),
                top: self.render_power(),
                middle: self.render_resistance(),
                bottom: self.render_statusbar(),
            },
            Mode::Settings(edit) => Ui {
                buttons: self.buttons.clone(),
                header: self.settings_header(edit),
                top: self.render_power(),
                middle: self.render_resistance(),
                bottom: self.render_brightness(),
            },
            Mode::Puffs(reset) => Ui {
                buttons: self.buttons.clone(),
                header: match reset {
                    ResetPuffs::None => Header::Tabs(Tab::Puffs),
                    ResetPuffs::Coil => Header::Title(RESET_COIL),
                    ResetPuffs::All => Header::Title(RESET_ALL),
                },
                top: Widget::PuffCoil {
                    duration: self.stats.coil,
                    reset: reset.is_coil() || reset.is_all(),
                },
                middle: Widget::PuffCount {
                    count: self.stats.count,
                    reset: reset.is_all(),
                },
                bottom: Widget::PuffTotal {
                    duration: self.stats.total,
                    reset: reset.is_all(),
                },
            },
            Mode::Battery => Ui {
                buttons: self.buttons.clone(),
                header: Header::Tabs(Tab::Battery),
                top: Widget::BatteryIdle(self.battery.idle),
                middle: Widget::BatteryLoad(self.battery.load),
                bottom: Widget::BatteryFull(self.battery.full),
            },
        };
        if new != self.ui {
            new.render(&self.ui, display);
            self.ui = new;
        }
    }
}

impl RendererImpl for State {

    fn settings_header(&self, edit: EditSettings) -> Header {
        let title = match edit {
            EditSettings::None => return Header::Tabs(Tab::Settings),
            EditSettings::Power => match self.config.power {
                Power::Rare => POWER_25,
                Power::Medium => POWER_50,
                Power::Well => POWER_75,
                Power::Hard => POWER_100,
            }
            EditSettings::Limit => LIMIT,
            EditSettings::Resistance => RESISTANCE,
            EditSettings::Brightness => BRIGHTNESS,
        };
        return Header::Title(title)
    }

    fn render_power(&self) -> Widget {
        Widget::PowerAndLimit {
            power: self.config.power.clone(),
            limit: self.config.limit,
            edit: match self.mode {
                Mode::Settings(EditSettings::Power) => power_and_limit::Edit::Power,
                Mode::Settings(EditSettings::Limit) => power_and_limit::Edit::Limit,
                _ => power_and_limit::Edit::None,
            },
        }
    }

    fn render_resistance(&self) -> Widget {
        Widget::ResistanceAndWatt {
            resistance: self.config.resistance,
            mw: self.battery.load.map(|load| self.config.milliwatts(load)),
            power: self.config.power.clone(),
            edit: matches!(self.mode, Mode::Settings(EditSettings::Resistance)),
        }
    }

    fn render_statusbar(&self) -> Widget {
        Widget::Statusbar {
            total: self.stats.total + self.calc_puff_duration(),
            battery: self.battery.clone(),
        }
    }

    fn render_brightness(&self) -> Widget {
        Widget::Brightness {
            brightness: self.config.brightness,
            focused: matches!(self.mode, Mode::Settings(EditSettings::Brightness)),
        }
    }
}
