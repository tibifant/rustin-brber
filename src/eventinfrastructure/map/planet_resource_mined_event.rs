use serde::{Deserialize, Serialize};

use crate::domainprimitives::location::{mineable_resource::MineableResource, mineable_resource_type::MineableResourceType};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetResourceMinedEvent {
    pub planet: String,
    pub mined_amount: u32,
    pub resource: Resource,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(rename = "type")]
    pub resource_type: MineableResourceType,
    pub max_amount: u32,
    pub current_amount: u32,
}
