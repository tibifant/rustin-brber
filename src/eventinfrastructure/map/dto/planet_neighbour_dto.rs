use serde::{Deserialize, Serialize};

use crate::domainprimitives::location::compass_direction_dto::CompassDirection;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanetNeighbourDto {
    #[serde(rename = "id")]
    pub planet_id: String,
    #[serde(rename = "direction")]
    pub compass_direction: CompassDirection,
}
