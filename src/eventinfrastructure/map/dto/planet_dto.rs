use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::map::dto::resource_type_dto::ResourceTypeDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetDto {
    pub planet_id: String,
    pub game_world_id: String,
    pub movement_difficulty: u8,
    pub resource_type: ResourceTypeDto,
}
