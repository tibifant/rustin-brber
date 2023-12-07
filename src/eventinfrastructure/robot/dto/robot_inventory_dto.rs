use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::robot::dto::robot_resource_inventory_dto::RobotResourceInventoryDto;
use crate::eventinfrastructure::robot::robot_level::RobotLevel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotInventoryDto {
    storage_level: RobotLevel,
    used_storage: u16,
    max_storage: u16,
    full: bool,
    resources: RobotResourceInventoryDto,
}
