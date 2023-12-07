use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::map::dto::resource_type_dto::ResourceTypeDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetResourceDto{
    resource_type: ResourceTypeDto,
    max_amount: u32,
    current_amount: u32
}