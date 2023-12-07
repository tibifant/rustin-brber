use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::robot::dto::robot_restoration_type::RobotRestorationType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotRestoredAttributesEvent {
    robot_id: String,
    restoration_type: RobotRestorationType,
    available_energy: u16,
    available_health: u16,
}