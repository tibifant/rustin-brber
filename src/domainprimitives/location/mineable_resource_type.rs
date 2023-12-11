use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MineableResourceType {
    COAL,
    IRON,
    GEM,
    GOLD,
    PLATIN,
}