use strum::EnumIs;

#[derive(Clone, PartialEq, EnumIs)]
pub enum Tab {
    Settings,
    Puffs,
    Battery,
}