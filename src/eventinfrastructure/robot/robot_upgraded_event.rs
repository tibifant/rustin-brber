use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::dto::robot_dto::RobotDto;
use crate::eventinfrastructure::robot::dto::robot_upgrade_type::RobotUpgradeType;
use crate::eventinfrastructure::robot::robot_level::RobotLevel;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotUpgradedEvent {
    pub robot_id: String,
    pub level: RobotLevel,
    pub upgrade: RobotUpgradeType,
    pub robot: RobotDto
}

