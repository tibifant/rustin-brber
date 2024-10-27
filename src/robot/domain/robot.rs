use crate::domainprimitives::command::action::Action;
use crate::domainprimitives::purchasing::robot_level::RobotLevel;
use crate::repository::Identifiable;

#[derive(Debug, Clone)]
pub struct TransientRobotInfo {
  pub id: String,
  pub planet_id: String,

  pub energy: u16,
  pub health: u16,

  pub health_level: RobotLevel,
  pub damage_level: RobotLevel,
  pub mining_speed_level: RobotLevel,
  pub mining_level: RobotLevel,
  pub energy_level: RobotLevel,
  pub energy_regen_level: RobotLevel,
  pub storage_level: RobotLevel,
}

#[derive(Debug, Clone)]
pub struct Inventory {
  pub coal: u16,
  pub iron: u16,
  pub gold: u16,
  pub gem: u16,
  pub platin: u16,
  pub full: bool,
  pub used_storage: u16,
  pub max_storage: u16,
}

#[derive(Debug, Clone)]
pub struct Robot {
  pub robot_info: TransientRobotInfo,
  pub inventory: Inventory,
  pub max_health: u16,
  pub max_energy: u16,
  pub energy_regen: u16,
  pub attack_damage: u16,
  pub mining_speed: u16,
  pub player_id: String
}

impl TransientRobotInfo {
  pub fn new(id: String, planet_id: String, energy: u16, health: u16, health_level: RobotLevel, damage_level: RobotLevel, mining_speed_level: RobotLevel, mining_level: RobotLevel, energy_level: RobotLevel, energy_regen_level: RobotLevel, storage_level: RobotLevel) -> Self {
    Self {
      id,
      planet_id,
      energy,
      health,
      health_level,
      damage_level,
      mining_speed_level,
      mining_level,
      energy_level,
      energy_regen_level,
      storage_level,
    }
  }
}

impl Inventory {
  pub fn new(coal: u16, iron: u16, gold: u16, gem: u16, platin: u16, full: bool, used_storage: u16, max_storage: u16) -> Self {
    Self {
      coal,
      iron,
      gold,
      gem,
      platin,
      full,
      used_storage,
      max_storage,
    }
  }
}

pub struct PersistentRobotInfo {
  pub id: String,
  pub player_id: String,
  pub max_health: u16,
  pub max_energy: u16,
  pub energy_regen: u16,
  pub attack_damage: u16,
  pub mining_speed: u16,
  pub inventory: Inventory,
  pub move_count: u16,   
}

impl PersistentRobotInfo {
  pub fn new(id: String, player_id: String, max_health: u16,max_energy: u16, energy_regen: u16, attack_damage: u16, mining_speed: u16, inventory: Inventory) -> Self {
    let move_count = 0;
    Self {
      id,
      player_id,
      max_health,
      max_energy,
      energy_regen,
      attack_damage,
      mining_speed,
      inventory,
      move_count,
    }
  }
}

pub struct RobotDecisionInfo {
  pub id: String,
  pub action: Box<dyn Action + Send + Sync>,
  pub upgrade_action: Box<dyn Action + Send + Sync>,
  pub has_upgrade: bool,
}

impl RobotDecisionInfo {
  pub fn new(id: String, action: Box<dyn Action + Send + Sync>, upgrade_action: Box<dyn Action + Send + Sync>, has_upgrade: bool) -> Self {
    Self {
      id,
      action,
      upgrade_action,
      has_upgrade,
    }
  }
}

impl Robot {
  pub fn new(robot_info: TransientRobotInfo, inventory: Inventory, max_health: u16, max_energy: u16, energy_regen: u16, attack_damage: u16, mining_speed: u16, player_id: String) -> Self {
    Self {
      robot_info,
      inventory,
      max_health,
      max_energy,
      energy_regen,
      attack_damage,
      mining_speed,
      player_id,
    }
  }

  pub fn check_health(&self, robot_info: TransientRobotInfo) -> bool {
    if robot_info.health == 0 {
      return false;
    }

    return true;
  }

  pub fn update(&mut self, robot_info: TransientRobotInfo) {
    self.robot_info = robot_info;
  }
}

impl Identifiable for Robot {
    fn id(&self) -> String {
        return self.robot_info.id.clone();
    }
}