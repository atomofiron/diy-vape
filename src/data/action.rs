use crate::data::button::Button;

pub enum Action {
    Power(Button),
    Limit(Button),
    Resistance(Button),
    Brightness(Button),
}
