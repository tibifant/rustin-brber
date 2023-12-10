use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::map::dto::resource_type_dto::ResourceTypeDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetResourceDto{
    pub resource_type: ResourceTypeDto,
    pub max_amount: u32,
    pub current_amount: u32
}