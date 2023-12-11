use serde::{Deserialize, Serialize};
use crate::domainprimitives::robot_level::RobotLevel;
use crate::domainprimitives::robot_upgrade_type::RobotUpgradeType;

use crate::eventinfrastructure::robot::dto::robot_dto::RobotDto;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotUpgradedEvent {
    pub robot_id: String,
    pub level: RobotLevel,
    pub upgrade: RobotUpgradeType,
    pub robot: RobotDto
}

