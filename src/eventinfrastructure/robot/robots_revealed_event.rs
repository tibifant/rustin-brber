use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::robot::dto::robots_revealed_robot_dto::RobotsRevealedRobotDto;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct RobotsRevealedEvent {
    pub robots: Vec<RobotsRevealedRobotDto>,
}


