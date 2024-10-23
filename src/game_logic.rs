use std::{collections::HashMap, hash::Hash};

use amqprs::connection;
use mockall::predicate::f32;

use crate::{rest::{game_service_rest_adapter_impl::{self, GameServiceRestAdapterImpl}, game_service_rest_adapter_trait::GameServiceRestAdapterTrait}, robot::domain::robot::{Inventory, MinimalRobot, Robot}};

pub enum TradeItemType {
  MiningSpeed1,
  MiningSpeed2,
  MiningSpeed3,
  MiningSpeed4,
  MiningSpeed5,
  MaxEnergy1,
  MaxEnergy2,
  MaxEnergy3,
  MaxEnergy4,
  MaxEnergy5,
  EnergyRegen1,
  EnergyRegen2,
  EnergyRegen3,
  EnergyRegen4,
  EnergyRegen5,
  Storage1,
  Storage2,
  Storage3,
  Storage4,
  Storage5,
  Mining1,
  Mining2,
  Mining3,
  Mining4,
  Mining5,
  Health1,
  Health2,
  Health3,
  Health4,
  Health5,
  Damage1,
  Damage2,
  Damage3,
  Damage4,
  Damage5,
  EnergyRestore,
  HealthRestore,
  Robot,
}

pub enum RessourceType {
  Gold,
  Coal,
  Platin,
  Iron,
  Gem,
}

pub struct Planet {
  pub id: String,
  pub movement_difficulty: u16,
  pub ressource: RessourceType,
  pub max_amount: u64,
  pub current_amount: u64,
  pub north: String,
  pub east: String,
  pub west: String,
  pub south: String,
}

pub enum RobotState {
  Mining,
  Attacking,
  // idk that's not what is supposed to be here....
}

pub struct PermanetRobotInfo {
  pub id: String,
  pub action: Action,
}

pub struct RoundData {
  pub robots: HashMap<String, Robot>,
  pub enemy_robots: HashMap<String, Robot>, //idk we can add the robot when it spawns as full robot i guess...
  pub planets: HashMap<String, Planet>,
  pub balance: i64,
  pub item_prices: HashMap<TradeItemType, i64>,
  pub ressource_prices: HashMap<RessourceType, i64>,
}

pub struct GameData {
  pub planets: HashMap<String, Planet>,
  pub robots: HashMap<String, PermanetRobotInfo>,
  pub player_id: String,
  // idk what else?
}

pub enum Direction {
  Here,
  North,
  East,
  South,
  West,
}

pub struct GameLogic {
  pub round_data: RoundData,
  pub game_data: GameData,
}

impl GameLogic {
  pub fn round_move(&self) {
    for (id, robot) in &self.game_data.robots {
      robot.action = NoneAction; // reset robot decision
    }

    self.offer_movement_mining_attack_option(robot);
    offer_sell_option(robot);
  }

  pub fn offer_movement_mining_attack_option(&self, robot_info: PermanetRobotInfo) {
    let robot = self.round_data.robots.get(robot_info.id.as_str());
    let planet = self.game_data.planets.get(robot.robot_info.planet_id);

    for (e_id, e) in &self.round_data.enemy_robots {
      if e.planet_id == robot.planet_id {
        let weight = (robot.attack_damage * robot.robot_info.damage_level - e.attack_damage * e.robot_info.damage_level);
        
        if robot_info.action.get_weight() < weight {
          let attack_option = AttackAction::new(weight, e_id);
          robot_info.action = attack_option;
        }
        break;
      }
    }

    let mut best_planet = Direction::Here;
    let mut best_price = self.round_data.resource_prices.get(planet.resource);
    let mut best_planet_amount = planet.current_amount;

    if (robot.energy > 0) {
      let north_planet = self.game_data.planets.get(planet.north);
      if (north_planet.has_value()) {
        let north_price = self.round_data.resource_prices.get(planet.id);
        if (north_price > best_price) {
          best_price = north_price;
          best_planet = Direction::North;
          best_planet_amount = north_planet.current_amount;
        }
      }

      let east_planet= self.game_data.planets.get(planet.east);
      if (east_planet.has_value()) {
        let east_price = self.round_data.resource_prices.get(planet.id);
        if (east_price > best_price) {
          best_price = east_price;
          best_planet = Direction::East;
          best_planet_amount = east_planet.current_amount;
        }
      }

      let south_planet= self.game_data.planets.get(planet.south);
      if (south_planet.has_value()) {
        let south_price = self.round_data.resource_prices.get(planet.id);
        if (south_price > best_price) {
          best_price = south_price;
          best_planet = Direction::South;
          best_planet_amount = south_planet.current_amount;
        }
      }

      let west_planet= self.game_data.planets.get(planet.west);
      if (west_planet.has_value()) {
        let west_price = self.round_data.resource_prices.get(planet.id);
        if (west_price > best_price) {
          best_price = west_price;
          best_planet = Direction::West;
          best_planet_amount = west_planet.current_amount;
        }
      }

      if best_planet != planet {
        let weight = best_price + best_planet_amount;

        if robot_info.action.get_weight() < weight {
          let movement_option = MovementAction::new(weight, best_planet);
          robot_info.action = movement_option;
        }
      }
      else {
        let weight = planet.current_amount + best_price;

        if robot_info.action.get_weight() < weight {
          let mining_option = MiningAction::new(weight, planet.id);
          robot_info.action = mining_option;
        }
      }
    }
  }
}

pub trait Action {
  fn get_weight(&self) -> f32;
  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String);
}

pub struct MovementAction {
  pub weight: f32,
  pub dir: Direction,
}

impl MovementAction {
  fn new(weight: f32, dir: Direction) -> Self {
    Self {
      dir,
      weight,
    }
  }
}

impl Action for MovementAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String) {
      
  }
}

pub struct AttackAction {
  pub weight: f32,
  pub target_robot: String,
}

impl AttackAction {
  fn new(weight: f32, target_robot: String) -> Self {
    Self {
      weight,
      target_robot,
    }
  }

} 

impl Action for AttackAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String) {
      
  }
}

pub struct SellAction {
  pub weight: f32,
}

impl SellAction {
  fn new(weight: f32) -> Self {
    Self {
      weight,
    }
  }
}

impl Action for SellAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String) {
      
  }
}

pub struct MineAction {
  pub weight: f32,
  pub target_planet_id: String,
}

impl MineAction {
  fn new(weight: f32, target_planet_id: String) -> Self {
    Self {
      weight,
      target_planet_id,
    }
  }
}

impl Action for MineAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String) {
      
  }
}

pub struct PurchaseAction {
  pub weight: f32,
  pub item: TradeItemType,
}

impl PurchaseAction {
  fn new(weight: f32, item: TradeItemType) -> Self {
    Self {
      weight,
      item,
    }
  }
}

impl Action for PurchaseAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String) {
      
  }
}

pub struct NoneAction {
  weight: f32,
}

impl NoneAction {
  fn new(weight: f32) -> Self {
    let weight = 0;

    Self {
      weight,
    }
  }
}

impl Action for NoneAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  fn execute_command(game_service_rest_adapter: GameServiceRestAdapterTrait, player_id: String, robot_id: String) {
      return;
  }
}
