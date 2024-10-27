use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::{domainprimitives::{command::command::Command, location::direction::Direction, purchasing::{robot_level::RobotLevel, robot_upgrade::RobotUpgrade, robot_upgrade_type::RobotUpgradeType, trade_item_type::TradeItemType}}, planet::domain::planet::PersistentPlanetInfo, rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait};

#[async_trait]
pub trait Action: Send + Sync {
  fn get_weight(&self) -> f32;
  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String);
}

pub struct MovementAction {
  pub weight: f32,
  pub dir: Direction,
  pub current_planet: PersistentPlanetInfo,
}

impl MovementAction {
  pub fn new(weight: f32, dir: Direction, current_planet: PersistentPlanetInfo) -> Self {
    Self {
      dir,
      weight,
      current_planet,
    }
  }
}

#[async_trait]
impl Action for MovementAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
    let mut planet_id = String::new();
    
    match self.dir {
      Direction::East => planet_id = self.current_planet.east.clone(),
      Direction::North => planet_id = self.current_planet.north.clone(),
      Direction::South => planet_id = self.current_planet.south.clone(),
      Direction::West => planet_id = self.current_planet.west.clone(),
      _ => return,
    }

    let command = Command::create_movement_command(player_id, robot_id.clone(), planet_id.clone());
    info!("====> Trying to move!!!!!!!!!!!");
    info!("robot ({}) moves from ({}) to ({})", robot_id.clone(), self.current_planet.id, planet_id.clone());
    game_service_rest_adapter.send_command(command).await;
  }
}

pub struct AttackAction {
  pub weight: f32,
  pub target_robot: String,
}

impl AttackAction {
  pub fn new(weight: f32, target_robot: String) -> Self {
    Self {
      weight,
      target_robot,
    }
  }

} 

#[async_trait]
impl Action for AttackAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      let command = Command::create_robot_attack_command(player_id, robot_id.clone(), self.target_robot.clone());
      info!("====> Trying to Attack!!!!!!!!!!!");
      info!("robot ({}) attacks robot ({})", robot_id.clone(), self.target_robot);
      game_service_rest_adapter.send_command(command).await;
  }
}

pub struct RegenerateAction {
  pub weight: f32,
}

impl RegenerateAction {
  pub fn new(weight: f32) -> Self {
    Self {
      weight,
    }
  }
} 

#[async_trait]
impl Action for RegenerateAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      let command = Command::create_robot_regenerate_command(player_id, robot_id.clone());
      info!("====> Trying to Regenerate!!!!!!!!!!!");
      info!("robot ({}) regenerates", robot_id.clone());
      game_service_rest_adapter.send_command(command).await;
  }
}

pub struct SellAction {
  pub weight: f32,
}

impl SellAction {
  pub fn new(weight: f32) -> Self {
    Self {
      weight,
    }
  }
}

#[async_trait]
impl Action for SellAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
    let command = Command::create_robot_sell_inventory_command(player_id, robot_id.clone());
    info!("====> Trying to Sell Inventory!!!!!!!!!!!");
    info!("robot ({}) sells inventory", robot_id.clone());
    game_service_rest_adapter.send_command(command).await;
  }
}

pub struct MineAction {
  pub weight: f32,
  pub target_planet_id: String,
}

impl MineAction {
  pub fn new(weight: f32, target_planet_id: String) -> Self {
    Self {
      weight,
      target_planet_id,
    }
  }
}

#[async_trait]
impl Action for MineAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
    let command = Command::create_robot_mine_command(player_id, robot_id.clone(), self.target_planet_id.clone());
    info!("====> Trying to Mine!!!!!!!!!!!");
    info!("robot ({}) mines resource on planet ({})", robot_id.clone(), self.target_planet_id.clone());
    game_service_rest_adapter.send_command(command).await;
  }
}

pub struct PurchaseAction {
  pub weight: f32,
  pub item: TradeItemType,
}

impl PurchaseAction {
  pub fn new(weight: f32, item: TradeItemType) -> Self {
    Self {
      weight,
      item,
    }
  }
}

#[async_trait]
impl Action for PurchaseAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
    let mut command = None;

    match self.item {
      TradeItemType::EnergyRestore => {
          command = Some(Command::create_robot_purchase_energy_restore_command(player_id, robot_id));
      }
      TradeItemType::HealthRestore => {
          command = Some(Command::create_robot_purchase_health_restore_command(player_id, robot_id));
      }
      TradeItemType::Damage1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Damage, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Damage2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Damage, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Damage3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Damage, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Damage4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Damage, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Damage5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Damage, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Health1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Health, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Health2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Health, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Health3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Health, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Health4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Health, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Health5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Health, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MiningSpeed1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MiningSpeed, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MiningSpeed2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MiningSpeed, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MiningSpeed3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MiningSpeed, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MiningSpeed4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MiningSpeed, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MiningSpeed5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MiningSpeed, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Mining1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Mining, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Mining2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Mining, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Mining3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Mining, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Mining4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Mining, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Mining5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Mining, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MaxEnergy1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MaxEnergy, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MaxEnergy2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MaxEnergy, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MaxEnergy3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MaxEnergy, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MaxEnergy4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MaxEnergy, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::MaxEnergy5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::MaxEnergy, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::EnergyRegen1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::EnergyRegen, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::EnergyRegen2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::EnergyRegen, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::EnergyRegen3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::EnergyRegen, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::EnergyRegen4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::EnergyRegen, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::EnergyRegen5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::EnergyRegen, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Storage1 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Storage, RobotLevel::LEVEL1);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Storage2 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Storage, RobotLevel::LEVEL2);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Storage3 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Storage, RobotLevel::LEVEL3);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Storage4 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Storage, RobotLevel::LEVEL4);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      TradeItemType::Storage5 => {
          let upgrade = RobotUpgrade::new(RobotUpgradeType::Storage, RobotLevel::LEVEL5);
          command = Some(Command::create_robot_upgrade_command(player_id, robot_id, &upgrade));
      }
      _ => {},
    }
    
    info!("====> Trying to Purchase Item!!!!!!!!!!!");
    
    match command {
      Some(command) => { game_service_rest_adapter.send_command(command).await; },
      None => {},
    }
  }
}

pub struct NoneAction {
  weight: f32,
}

impl NoneAction {
  pub fn new() -> Self {
    let weight = 0.;

    Self {
      weight,
    }
  }
}

#[async_trait]
impl Action for NoneAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      return;
  }
}

pub async fn execute_purchase_robots_command(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, amount: u16) {
  let buy_robot_command = Command::create_robot_purchase_command(player_id, amount);
  info!("====> Try to buy Robots!!!!!!!!!!!!!!.");
  let _ = game_service_rest_adapter.send_command(buy_robot_command).await;
}
