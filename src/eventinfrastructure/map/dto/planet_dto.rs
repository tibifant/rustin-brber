use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::map::dto::resource_type_dto::ResourceTypeDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetDto {
    planet_id: String,
    game_world_id: String,
    movement_difficulty: u8,
    resource_type: ResourceTypeDto,
}
