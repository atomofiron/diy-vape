use strum::EnumIs;

#[derive(Clone, PartialEq, EnumIs)]
pub enum ResetPuffs {
    None,
    Coil,
    All,
}
