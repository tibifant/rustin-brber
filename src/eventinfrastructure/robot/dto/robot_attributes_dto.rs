use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobotAttributesDto {
    pub max_health: u16,
    pub max_energy: u16,
    pub energy_regen: u16,
    pub attack_damage: u16,
    pub mining_speed: u16,
    pub health: u16,
    pub energy: u16,
}