use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::robot::dto::robot_attack_info_dto::RobotAttackInfoDto;

#[derive(Serialize, Deserialize,Debug)]
pub struct RobotAttackedEvent {
    pub attacker : RobotAttackInfoDto,
    pub target : RobotAttackInfoDto,
}