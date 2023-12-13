use serde::{Deserialize, Serialize};

use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;
use crate::eventinfrastructure::robot::dto::robot_resource_inventory_dto::RobotResourceInventoryDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotResourceMinedEvent {
    pub robot_id: String,
    pub mined_amount: u16,
    pub mined_resource: MineableResourceType,
    pub resource_inventory: RobotResourceInventoryDto,
}
