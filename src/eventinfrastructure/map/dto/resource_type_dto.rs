use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceTypeDto {
    COAL,
    IRON,
    GEM,
    GOLD,
    PLATIN,
}