use serde::{Deserialize, Serialize};

use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MineableResource {
    pub resource_type: MineableResourceType,
    pub max_amount: u32,
    pub current_amount: u32
}