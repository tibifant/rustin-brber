use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::dto::robot_resource_inventory_dto::RobotResourceInventoryDto;
use crate::eventinfrastructure::robot::robot_level::RobotLevel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotInventoryDto {
    pub storage_level: RobotLevel,
    pub used_storage: u16,
    pub max_storage: u16,
    pub full: bool,
    pub resources: RobotResourceInventoryDto,
}
