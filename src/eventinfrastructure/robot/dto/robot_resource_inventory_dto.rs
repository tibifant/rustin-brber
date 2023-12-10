use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct RobotResourceInventoryDto {
    pub coal: u16,
    pub iron: u16,
    pub gold: u16,
    pub gem: u16,
    pub platin: u16
}