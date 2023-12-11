use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::map::dto::planet_dto::PlanetDto;
use crate::eventinfrastructure::robot::dto::robot_attributes_dto::RobotAttributesDto;
use crate::eventinfrastructure::robot::dto::robot_inventory_dto::RobotInventoryDto;
use crate::eventinfrastructure::robot::robot_level::RobotsRevealedLevelDto;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotDto {
    #[serde(rename = "id")]
    pub robot_id: String,
    pub alive: bool,
    #[serde(rename = "player")]
    pub player_id: String,
    pub planet: PlanetDto,
    pub inventory: RobotInventoryDto,
    #[serde(flatten)]
    pub robot_attributes: RobotAttributesDto,
    #[serde(flatten)]
    pub robot_levels : RobotsRevealedLevelDto,
}