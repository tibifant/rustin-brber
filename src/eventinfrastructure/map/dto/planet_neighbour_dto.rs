use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::map::dto::compass_direction_dto::CompassDirectionDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanetNeighbourDto {
    #[serde(rename = "id")]
    planet_id: String,
    #[serde(rename = "direction")]
    compass_direction: CompassDirectionDto,
}

