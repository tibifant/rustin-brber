use crate::domainprimitives::purchasing::robot_level::RobotLevel;
use crate::repository::Identifiable;

#[derive(Debug, Clone)]
pub struct MinimalRobot {
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
  pub robot_info: MinimalRobot,
  pub inventory: Inventory,
  pub max_health: u16,
  pub max_energy: u16,
  pub energy_regen: u16,
  pub attack_damage: u16,
  pub mining_speed: u16,
  pub player_id: String
}

impl MinimalRobot {
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

impl Robot {
  pub fn new(robot_info: MinimalRobot, inventory: Inventory, max_health: u16, max_energy: u16, energy_regen: u16, attack_damage: u16, mining_speed: u16, player_id: String) -> Self {
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

  pub fn check_health(&self, robot_info: MinimalRobot) -> bool {
    if robot_info.health == 0 {
      return false;
    }

    return true;
  }

  pub fn update(&mut self, robot_info: MinimalRobot) {
    self.robot_info = robot_info;
  }
}

impl Identifiable for Robot {
    fn id(&self) -> String {
        return self.robot_info.id.clone();
    }
}