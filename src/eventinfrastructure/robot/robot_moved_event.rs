use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::dto::robot_move_planet_info_dto::RobotMovePlanetInfoDto;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotMovedEvent {
    pub robot_id : String,
    pub remaining_energy : u16,
    pub from_planet : RobotMovePlanetInfoDto,
    pub to_planet : RobotMovePlanetInfoDto,
}

