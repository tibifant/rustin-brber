use serde::{Deserialize, Serialize};
use crate::domainprimitives::location::mineable_resource::MineableResource;

use crate::eventinfrastructure::map::dto::planet_neighbour_dto::PlanetNeighbourDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetDiscoveredEvent{
    pub planet_id: String,
    pub movement_difficulty: u8,
    pub neighbours: Vec<PlanetNeighbourDto>,
    pub resource: MineableResource
}