use serde::{Deserialize, Serialize};
use crate::domainprimitives::robot_level::RobotLevel;

use crate::eventinfrastructure::robot::dto::robot_resource_inventory_dto::RobotResourceInventoryDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotInventoryDto {
    pub storage_level: RobotLevel,
    pub used_storage: u16,
    pub max_storage: u16,
    pub full: bool,
    pub resources: RobotResourceInventoryDto,
}
