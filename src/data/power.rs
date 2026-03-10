use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MaxSize, Clone, Debug)]
pub enum Power {
    Rare,
    Medium,
    Well,
    Hard,
}

impl Power {

    pub fn value(&self) -> u8 {
        match self {
            Self::Rare => 25,
            Self::Medium => 50,
            Self::Well => 75,
            Self::Hard => 100,
        }
    }

    pub fn inc(&self) -> Power {
        match self {
            Self::Rare => Power::Medium,
            Self::Medium => Power::Well,
            Self::Well => Power::Hard,
            Self::Hard => Power::Hard,
        }
    }

    pub fn dec(&self) -> Power {
        match self {
            Self::Rare => Power::Rare,
            Self::Medium => Power::Rare,
            Self::Well => Power::Medium,
            Self::Hard => Power::Well,
        }
    }
}

impl Default for Power {
    fn default() -> Self {
        Self::Medium
    }
}
