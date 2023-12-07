use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::map::dto::planet_resource_dto::PlanetResourceDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetResourceMinedEvent {
    planet_id: String,
    mined_amount: u32,
    resource: PlanetResourceDto
}