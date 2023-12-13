use serde::{Deserialize, Serialize};

use crate::domainprimitives::purchasing::robot_level::RobotLevel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedLevelDto {
    pub health_level: RobotLevel,
    pub energy_level: RobotLevel,
    pub energy_regen_level: RobotLevel,
    pub damage_level: RobotLevel,
    pub mining_speed_level: RobotLevel,
    pub mining_level: RobotLevel,
    pub storage_level: RobotLevel,
}
