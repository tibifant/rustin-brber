use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum MineableResourceType {
    COAL,
    IRON,
    GEM,
    GOLD,
    PLATIN,
}

impl Display for MineableResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineableResourceType::COAL => write!(f, "COAL"),
            MineableResourceType::IRON => write!(f, "IRON"),
            MineableResourceType::GEM => write!(f, "GEM"),
            MineableResourceType::GOLD => write!(f, "GOLD"),
            MineableResourceType::PLATIN => write!(f, "PLATIN"),
        }
    }
}
