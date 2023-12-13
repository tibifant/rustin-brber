use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::dto::robot_dto::RobotDto;

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotSpawnedEvent {
    pub robot: RobotDto,
}
