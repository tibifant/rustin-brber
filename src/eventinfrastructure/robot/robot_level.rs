use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domainprimitives::purchasing::robot_level::RobotLevel;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedLevelDto {
    pub damage_level: RobotLevel,
    pub energy_level: RobotLevel,
    pub energy_regen_level: RobotLevel,
    pub health_level: RobotLevel,
    pub mining_level: RobotLevel,
    pub mining_speed_level: RobotLevel,
    #[serde(default = "RobotLevel::default_level")]
    pub storage_level: RobotLevel,
}
