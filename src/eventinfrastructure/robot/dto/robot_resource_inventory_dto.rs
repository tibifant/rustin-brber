use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct RobotResourceInventoryDto {
    coal: u16,
    iron: u16,
    gold: u16,
    gem: u16,
    platin: u16
}