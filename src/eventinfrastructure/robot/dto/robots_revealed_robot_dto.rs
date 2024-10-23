use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::robot_level::RobotsRevealedLevelDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedRobotDto {
    pub energy: u16,
    pub health: u16,
    pub levels: RobotsRevealedLevelDto,
    pub robot_id: String,
    pub planet_id: String,
    pub player_notion: String,
}
