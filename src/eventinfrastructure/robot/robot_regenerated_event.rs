use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotRegeneratedEvent {
    robot_id: String,
    available_energy: u16,
}