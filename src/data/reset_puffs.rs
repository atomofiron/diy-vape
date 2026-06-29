use strum::EnumIs;

#[derive(Clone, PartialEq, EnumIs)]
pub enum ResetPuffs {
    None,
    Coil,
    Count,
    Total,
}

impl ResetPuffs {

    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Coil,
            Self::Coil => Self::Count,
            Self::Count => Self::Total,
            Self::Total => Self::None,
        }
    }
}
