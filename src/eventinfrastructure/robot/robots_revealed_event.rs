use serde::{Deserialize, Serialize, Deserializer};
use crate::eventinfrastructure::robot::robot_level::RobotLevels;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedEvent {
    pub robots: Vec<RobotsRevealedRobotDto>,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedRobotDto {
    pub robot_id : String,
    pub planet_id : String,
    pub player_notion : String,
    pub levels : RobotLevels,
    pub health : u16,
    pub energy : u16,
}
