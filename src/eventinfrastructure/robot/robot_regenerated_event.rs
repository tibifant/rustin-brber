use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotRegeneratedEvent {
    pub robot_id: String,
    pub available_energy: u16,
}