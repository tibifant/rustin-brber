use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domainprimitives::purchasing::robot_level::RobotLevel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedLevelDto {
    pub damage_level: u16,
    pub energy_level: u16,
    pub energy_regen_level: u16,
    pub health_level: u16,
    pub mining_level: u16,
    pub mining_speed_level: u16,
    #[serde(default)]
    pub storage_level: u16,
    #[serde(flatten)]
    extra: Option<Value>,
}
