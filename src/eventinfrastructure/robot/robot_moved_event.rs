use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::robot::dto::robot_move_planet_info_dto::RobotMovePlanetInfoDto;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotMovedEvent {
    robot_id : String,
    remaining_energy : u16,
    from_planet : RobotMovePlanetInfoDto,
    to_planet : RobotMovePlanetInfoDto,
}

