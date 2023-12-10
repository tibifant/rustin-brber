use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::map::dto::planet_neighbour_dto::PlanetNeighbourDto;
use crate::eventinfrastructure::map::dto::planet_resource_dto::PlanetResourceDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetDiscoveredEvent{
    pub planet_id: String,
    pub movement_difficulty: u8,
    pub neighbours: Vec<PlanetNeighbourDto>,
    pub resource: PlanetResourceDto
}