use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::map::dto::compass_direction_dto::CompassDirectionDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanetNeighbourDto {
    #[serde(rename = "id")]
    pub planet_id: String,
    #[serde(rename = "direction")]
    pub compass_direction: CompassDirectionDto,
}

