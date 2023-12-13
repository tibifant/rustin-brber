use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::dto::robot_restoration_type::RobotRestorationType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotRestoredAttributesEvent {
    pub robot_id: String,
    pub restoration_type: RobotRestorationType,
    pub available_energy: u16,
    pub available_health: u16,
}
