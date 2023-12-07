use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobotAttackInfoDto {
    robot_id: String,
    available_health : u16,
    available_energy : u16,
    alive : bool,
}