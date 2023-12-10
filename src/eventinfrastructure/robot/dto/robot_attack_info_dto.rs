use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobotAttackInfoDto {
    pub robot_id: String,
    pub available_health : u16,
    pub available_energy : u16,
    pub alive : bool,
}