use crate::eventinfrastructure::robot::dto::robot_attack_info_dto::RobotAttackInfoDto;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Debug)]
pub struct RobotAttackedEvent {
    attacker : RobotAttackInfoDto,
    target : RobotAttackInfoDto,
}