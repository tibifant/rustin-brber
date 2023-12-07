use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::map::dto::resource_type_dto::ResourceTypeDto;
use crate::eventinfrastructure::robot::dto::robot_resource_inventory_dto::RobotResourceInventoryDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotResourceRemovedEvent {
    robot_id: String,
    removed_amount: u16,
    removed_resource: ResourceTypeDto,
    resource_inventory: RobotResourceInventoryDto,
}