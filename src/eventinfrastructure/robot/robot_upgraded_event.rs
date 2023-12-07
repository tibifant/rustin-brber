use crate::eventinfrastructure::robot::robot_level::RobotLevel;
use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::robot::dto::robot_dto::RobotDto;
use crate::eventinfrastructure::robot::dto::robot_upgrade_type::RobotUpgradeType;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotUpgradedEvent {
    robot_id: String,
    level: RobotLevel,
    upgrade: RobotUpgradeType,
    robot: RobotDto
}

