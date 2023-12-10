use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::map::dto::planet_resource_dto::PlanetResourceDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetResourceMinedEvent {
    pub planet_id: String,
    pub mined_amount: u32,
    pub resource: PlanetResourceDto
}