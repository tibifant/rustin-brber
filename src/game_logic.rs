use std::f32::consts::E;
use std::{collections::HashMap, hash::Hash};

use std::sync::Arc;

use tokio::sync::Mutex;

use serde_json::de;
use tracing::info;

use async_trait::async_trait;

use crate::domainprimitives::command;
use crate::domainprimitives::command::command::Command;
use crate::domainprimitives::location::compass_direction_dto::CompassDirection;
use crate::domainprimitives::purchasing::robot_level::RobotLevel;
use crate::domainprimitives::purchasing::robot_upgrade::RobotUpgrade;
use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::map::planet_discovered_event::PlanetDiscoveredEvent;
use crate::eventinfrastructure::map::planet_resource_mined_event::PlanetResourceMinedEvent;
use crate::eventinfrastructure::robot;
use crate::eventinfrastructure::robot::robot_resource_mined_event::RobotResourceMinedEvent;
use crate::eventinfrastructure::robot::robot_resource_removed_event::RobotResourceRemovedEvent;
use crate::eventinfrastructure::robot::robots_revealed_event::RobotsRevealedEvent;
use crate::eventinfrastructure::robot::robot_spawned_event::RobotSpawnedEvent;
use crate::domainprimitives::purchasing::robot_upgrade_type::RobotUpgradeType;
use crate::eventinfrastructure::trading::bank_account_initialized_event::BankAccountInitializedEvent;
use crate::eventinfrastructure::trading::bank_account_transaction_booked::BankAccountTransactionBookedEvent;
use crate::eventinfrastructure::trading::tradable_prices_event::TradablePricesEvent;
use crate::{rest::{game_service_rest_adapter_impl::{self, GameServiceRestAdapterImpl}, game_service_rest_adapter_trait::GameServiceRestAdapterTrait}, robot::domain::robot::{Inventory, MinimalRobot, Robot}};
use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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

impl TradeItemType {
  pub fn get_next_level_item(upgrade_type: RobotUpgradeType, current_level: u16) -> Option<TradeItemType> {
    match upgrade_type {
        RobotUpgradeType::Storage => match current_level {
            1 => Some(TradeItemType::Storage2),
            2 => Some(TradeItemType::Storage3),
            3 => Some(TradeItemType::Storage4),
            4 => Some(TradeItemType::Storage5),
            _ => None,
        },
        RobotUpgradeType::Health => match current_level {
            1 => Some(TradeItemType::Health2),
            2 => Some(TradeItemType::Health3),
            3 => Some(TradeItemType::Health4),
            4 => Some(TradeItemType::Health5),
            _ => None,
        },
        RobotUpgradeType::Damage => match current_level {
            1 => Some(TradeItemType::Damage2),
            2 => Some(TradeItemType::Damage3),
            3 => Some(TradeItemType::Damage4),
            4 => Some(TradeItemType::Damage5),
            _ => None,
        },
        RobotUpgradeType::MiningSpeed => match current_level {
            1 => Some(TradeItemType::MiningSpeed2),
            2 => Some(TradeItemType::MiningSpeed3),
            3 => Some(TradeItemType::MiningSpeed4),
            4 => Some(TradeItemType::MiningSpeed5),
            _ => None,
        },
        RobotUpgradeType::Mining => match current_level {
            1 => Some(TradeItemType::Mining2),
            2 => Some(TradeItemType::Mining3),
            3 => Some(TradeItemType::Mining4),
            4 => Some(TradeItemType::Mining5),
            _ => None,
        },
        RobotUpgradeType::MaxEnergy => match current_level {
            1 => Some(TradeItemType::MaxEnergy2),
            2 => Some(TradeItemType::MaxEnergy3),
            3 => Some(TradeItemType::MaxEnergy4),
            4 => Some(TradeItemType::MaxEnergy5),
            _ => None,
        },
        RobotUpgradeType::EnergyRegen => match current_level {
            1 => Some(TradeItemType::EnergyRegen2),
            2 => Some(TradeItemType::EnergyRegen3),
            3 => Some(TradeItemType::EnergyRegen4),
            4 => Some(TradeItemType::EnergyRegen5),
            _ => None,
        },
    }
  }
}

pub struct Planet {
  pub id: String,
  pub current_amount: u32,
}

impl Planet {
  pub fn new(id: String, current_amount: u32) -> Self {
    Self {
      id,
      current_amount,
    }
  }
}

#[derive(Clone)]
pub struct PermanentPlanetInfo {
  pub id: String,
  pub movement_difficulty: u8,
  pub resource: MineableResourceType,
  pub max_amount: u32,
  pub last_known_amount: u32,
  pub north: String,
  pub east: String,
  pub west: String,
  pub south: String,                  
}

impl PermanentPlanetInfo {
  pub fn new(id: String, movement_difficulty: u8, resource: MineableResourceType, max_amount: u32, last_known_amount: u32, north: String, east: String, west: String, south: String) -> Self {
    Self {
      id,
      movement_difficulty,
      resource,
      max_amount,
      last_known_amount,
      north,
      east,
      west,
      south,
    }
  }
}

pub struct PermanetRobotInfo {
  pub id: String,
  pub player_id: String,
  pub max_health: u16,
  pub max_energy: u16,
  pub energy_regen: u16,
  pub attack_damage: u16,
  pub mining_speed: u16,
  pub inventory: Inventory,           
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

pub struct DecisionInfo {
  pub robots: HashMap<String, RobotDecisionInfo>
}

impl DecisionInfo {
  pub fn new() -> Self {
    let robots = HashMap::new();
    Self {
      robots
    }
  }
}

impl PermanetRobotInfo {
  pub fn new(id: String, player_id: String, max_health: u16,max_energy: u16, energy_regen: u16, attack_damage: u16, mining_speed: u16, inventory: Inventory) -> Self {
    Self {
      id,
      player_id,
      max_health,
      max_energy,
      energy_regen,
      attack_damage,
      mining_speed,
      inventory,       
    }
  }
}

pub struct RoundData {
  pub robots: HashMap<String, MinimalRobot>,
  pub enemy_robots: HashMap<String, MinimalRobot>, //TODO either make this MinimalRobot or have a PermanentRobot in the GameData with the other values
  pub planets: HashMap<String, Planet>,
  pub balance: f32,
  pub item_prices: HashMap<TradeItemType, f32>,
  pub resource_prices: HashMap<MineableResourceType, f32>,
}

impl RoundData {
  pub fn new() -> Self {
    let robots = HashMap::new();
    let enemy_robots = HashMap::new();
    let planets = HashMap::new();
    let balance = 0.;
    let item_prices = HashMap::new();
    let resource_prices = HashMap::new();

    Self {
      robots,
      enemy_robots,
      planets,
      balance,
      item_prices,
      resource_prices,
    }
  }
}

pub struct GameData {
  pub planets: HashMap<String, PermanentPlanetInfo>,
  pub robots: HashMap<String, PermanetRobotInfo>,
  pub player_id: String,
  pub robot_buy_amount: u16,
}

impl GameData {
  pub fn new() -> Self {
    let planets = HashMap::new();
    let robots = HashMap::new();
    let player_id = String::new();
    let robot_buy_amount = 0;
    Self {
      planets,
      robots,
      player_id,
      robot_buy_amount,
    }
  }
}

#[derive(PartialEq)]
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
  pub fn new() -> Self {
    let round_data = RoundData::new();
    let game_data = GameData::new();

    Self {
      round_data,
      game_data,
    }
  }

  pub async fn round_move(&mut self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>) {

    self.game_data.robot_buy_amount = 0;

    let mut decision_info = DecisionInfo::new();

    for (id, robot) in &mut self.game_data.robots {
      let a: Box<dyn Action + Send + Sync> = Box::new(NoneAction::new());
      let pa: Box<dyn Action + Send + Sync> = Box::new(NoneAction::new());
      let r = RobotDecisionInfo::new(id.clone(), a, pa, false);
      decision_info.robots.insert(id.clone(), r);
    }

    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    for id in ids {
      if let Some(r) = decision_info.robots.get_mut(&id) {
        self.offer_movement_mining_attack_option(id.to_string(), r);
        self.offer_sell_option(id.to_string(), r);
      }
    }
    
    while self.round_data.balance > 0. {
      if !self.spend_money(&mut decision_info) {
        break;
      }
    }

    for (_id, robot) in &mut decision_info.robots {
      robot.action.execute_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), robot.id.to_string()).await;
      robot.upgrade_action.execute_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), robot.id.to_string()).await;
    }

    if self.game_data.robot_buy_amount > 0 {
      execute_purchase_robots_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), self.game_data.robot_buy_amount).await;
    }
  }

  fn offer_movement_mining_attack_option(&mut self, robot_id: String, robot_decision: &mut RobotDecisionInfo) {
    if let Some(robot_info) = self.game_data.robots.get_mut(&robot_id) {
      if let Some(robot) = self.round_data.robots.get(&robot_id) {
        if robot.energy > 0 {
          let planet = self.game_data.planets.get(&robot.planet_id);
          
          match planet {
            Some(planet) => {
              for (e_id, e) in &self.round_data.enemy_robots {
                if e.planet_id == robot.planet_id {
                  let weight: f32 = (robot_info.attack_damage - e.damage_level.get_attack_damage_value_for_level()) as f32;
                
                  if robot_decision.action.get_weight() < weight {
                    let attack_option: Box<dyn Action + Send + Sync> = Box::new(AttackAction::new(weight, e_id.to_string()));
                    robot_decision.action = attack_option;
                  }
                  break;
                }
              }
            
              let mut best_planet = Direction::Here;
              let mut best_price = self.round_data.resource_prices.get(&planet.resource).unwrap_or(&0.);
              let mut best_planet_amount = planet.last_known_amount;
            
              if robot.energy > 0 {
                let north_planet = self.game_data.planets.get(&planet.north);
                if let Some(north_planet) = north_planet {
                  let north_price = self.round_data.resource_prices.get(&planet.resource).unwrap_or(&0.);
                  if (north_price > best_price) {
                    best_price = north_price;
                    best_planet = Direction::North;
                    best_planet_amount = north_planet.last_known_amount;
                  }
                }
              
                let east_planet= self.game_data.planets.get(&planet.east);
                if let Some(east_planet) = east_planet {
                  let east_price = self.round_data.resource_prices.get(&planet.resource).unwrap_or(&0.);
                  if (east_price > best_price) {
                    best_price = east_price;
                    best_planet = Direction::East;
                    best_planet_amount = east_planet.last_known_amount;
                  }
                }
              
                let south_planet= self.game_data.planets.get(&planet.south);
                if let Some(south_planet) = south_planet {
                  let south_price = self.round_data.resource_prices.get(&planet.resource).unwrap_or(&0.);
                  if (south_price > best_price) {
                    best_price = south_price;
                    best_planet = Direction::South;
                    best_planet_amount = south_planet.last_known_amount;
                  }
                }
              
                let west_planet= self.game_data.planets.get(&planet.west);
                if let Some(west_planet) = west_planet {
                  let west_price = self.round_data.resource_prices.get(&planet.resource).unwrap_or(&0.);
                  if (west_price > best_price) {
                    best_price = west_price;
                    best_planet = Direction::West;
                   best_planet_amount = west_planet.last_known_amount;
                 }
                }
              
                if best_planet_amount as f32 * best_price > 0. {
                  if best_planet != Direction::Here {
                    let weight = best_price + best_planet_amount as f32;
                  
                    if robot_decision.action.get_weight() < weight {
                      let movement_option: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(weight, best_planet, planet.clone()));
                      robot_decision.action = movement_option;
                    }
                  } else {
                    let weight = planet.last_known_amount as f32 + best_price;
                  
                    if robot_decision.action.get_weight() < weight {
                      let mining_option: Box<dyn Action + Send + Sync> = Box::new(MineAction::new(weight, planet.id.to_string()));
                      robot_decision.action = mining_option;
                    }
                  }
                }
                else {
                  if robot_decision.action.get_weight() < 2000. {
                    let a: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(2000., Direction::East, planet.clone()));
                    robot_decision.action = a;
                  }
                }
              }
            }
            None => {}
          }
        }
        else {
          let a: Box<dyn Action + Send + Sync> = Box::new(RegenerateAction::new(9999999.));
          robot_decision.action = a;
        }
      }
    }
  }

  fn offer_sell_option(&mut self, robot_id: String, robot_decision: &mut RobotDecisionInfo) {
    if let Some(robot_info) = self.game_data.robots.get_mut(&robot_id) {
      if let Some(robot) = self.round_data.robots.get(robot_info.id.as_str()) {
        let inventory_weight = 0.1 * ((robot_info.max_health - robot.health) as f32) * (robot_info.inventory.coal as f32) * self.round_data.resource_prices.get(&MineableResourceType::COAL).unwrap_or(&0.) + (robot_info.inventory.gem as f32) * self.round_data.resource_prices.get(&MineableResourceType::GEM).unwrap_or(&0.) + (robot_info.inventory.gold  as f32) * self.round_data.resource_prices.get(&MineableResourceType::GOLD).unwrap_or(&0.)+ (robot_info.inventory.iron as f32) * self.round_data.resource_prices.get(&MineableResourceType::IRON).unwrap_or(&0.) + (robot_info.inventory.platin as f32) * self.round_data.resource_prices.get(&MineableResourceType::PLATIN).unwrap_or(&0.);

        if inventory_weight > robot_decision.action.get_weight() {
          let inventory_option: Box<dyn Action + Send + Sync> = Box::new(SellAction::new(inventory_weight as f32));
          robot_decision.action = inventory_option;
        }
      }
    }
  }

  fn spend_money(&mut self, decision_info: &mut DecisionInfo) -> bool {
    let mut best_weight: f32 = 0.;
    let mut best_item = TradeItemType::Robot;
    let mut best_id: Option<String> = None;
    let mut is_upgrade = false;

    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    for id in ids {
      if let Some(robot_decision_info) = decision_info.robots.get(&id) {
        if let Some(robot_info) = self.game_data.robots.get(&id) {
          if let Some(robot) = self.round_data.robots.get(&id) {
            if !robot_decision_info.has_upgrade {
              if let Some(health_price) = self.round_data.item_prices.get(&TradeItemType::HealthRestore) {
                if self.round_data.balance >= *health_price {
                  let weight = (robot_info.max_health - robot.health) * robot.health_level.get_value_for_level() + robot.damage_level.get_value_for_level() + robot.mining_speed_level.get_value_for_level();
                  let item = TradeItemType::HealthRestore;
                
                  if weight as f32 > best_weight {
                    best_weight = weight as f32;
                    best_item = item;
                    best_id = Some(id.clone());
                    is_upgrade = false;
                  }
                
                  if best_weight < 50. && *health_price > 10. + self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&0.) && self.round_data.balance >= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&0.){
                    if (1000. > best_weight) {
                      best_item = TradeItemType::Robot;
                      best_weight = 1000.;
                      is_upgrade = false;
                    }
                  } 
                }
              }
              if let Some(energy_price) = self.round_data.item_prices.get(&TradeItemType::EnergyRestore) {
                if self.round_data.balance >= *energy_price {
                  let weight = (robot_info.max_energy - robot.energy) * robot.energy_level.get_value_for_level() * robot.damage_level.get_value_for_level() * robot.mining_speed_level.get_value_for_level();
                  let item = TradeItemType::EnergyRestore;
                
                  if weight as f32 > best_weight {
                    best_weight = weight as f32;
                    best_item = item;
                    best_id = Some(id.clone());
                    is_upgrade = false;
                  }
                  if best_weight < 50. && *energy_price > 10. + self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&0.) && self.round_data.balance >= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&0.) {
                    if (1000. > best_weight) {
                      best_item = TradeItemType::Robot;
                      best_weight = 1000.;
                      is_upgrade = false;
                    }
                  } 
                }
              }

              if let Some(planet) = self.game_data.planets.get(robot.planet_id.as_str()) {
                if !robot.storage_level.is_maximum_level() {
                  if let Some(next_level_item) = TradeItemType::get_next_level_item(RobotUpgradeType::Storage, robot.storage_level.get_value_for_level()) {
                    if let Some(item_price) = self.round_data.item_prices.get(&next_level_item) {
                      if self.round_data.balance >= *item_price {
                        if let Some(ressource_price) = self.round_data.resource_prices.get(&planet.resource) {
                          let weight = (planet.last_known_amount as f32) * ressource_price + robot_info.inventory.used_storage as f32;
                          if weight > best_weight as f32 {
                            best_weight = weight;
                            best_item = next_level_item;
                            best_id = Some(id.clone());
                            is_upgrade = true;
                          }
                        }
                      }
                    }
                  }
                }
              }
              // Posibility to implement other upgrade options here
            }
          }
        }
      }
    }

    if (self.round_data.balance >= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&1000000.)) {
      if 1000. >= best_weight {
        best_item = TradeItemType::Robot;
        best_weight = 1000.;
      }
    }

    if best_weight > 0. {
      if best_item == TradeItemType::Robot {
        self.round_data.balance -= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&10000000.);
        self.game_data.robot_buy_amount += 1;
        return true;
      }

      if let Some(id) = best_id {
        if let Some(best_robot) = decision_info.robots.get_mut(&id) {
          if !is_upgrade {
            if best_weight > best_robot.action.get_weight() {
              let a: Box<dyn Action + Send + Sync> = Box::new(PurchaseAction::new(best_weight as f32, best_item));
              best_robot.action = a;
              self.round_data.balance -= *self.round_data.item_prices.get(&(best_item.clone())).unwrap_or(&10000000.);
              return true;
            }
          }
          else {
            let a: Box<dyn Action + Send + Sync> = Box::new(PurchaseAction::new(best_weight as f32, best_item));
            best_robot.upgrade_action = a;
            best_robot.has_upgrade = true;
            self.round_data.balance -= *self.round_data.item_prices.get(&(best_item.clone())).unwrap_or(&10000000.);
            return true;
          }
        }
      }
    }
    return false;
  }

  // Event Stuff
  pub fn balance_update(&mut self, balance: f32) {
    self.round_data.balance = balance;
  }

  pub fn update_item_price(&mut self, item: TradeItemType, price: f32) {
    if let Some(p) = self.round_data.item_prices.get_mut(&item) {
      *p = price;
    }
    else {
      self.round_data.item_prices.insert(item, price);
    }
  }
  
  pub fn update_resource_price(&mut self, item: MineableResourceType, price: f32) {
    if let Some(mut p) = self.round_data.resource_prices.get_mut(&item) {
      *p = price;
    }
    else {
      self.round_data.resource_prices.insert(item, price);
    }
  }

  pub fn save_robot(&mut self, robot: Robot) {
    if robot.player_id == self.game_data.player_id {
      let new_robot_info = PermanetRobotInfo::new(robot.robot_info.id.clone(), robot.player_id, robot.max_health, robot.max_energy, robot.energy_regen, robot.attack_damage, robot.mining_speed, robot.inventory);
      
      self.game_data.robots.insert(new_robot_info.id.clone(), new_robot_info);      
      self.round_data.robots.insert(robot.robot_info.id.clone(), robot.robot_info);
    }
    else {
      self.round_data.enemy_robots.insert(robot.robot_info.id.clone(), robot.robot_info);
    }
  }

  pub fn update_robot(&mut self, updated_robot: &mut MinimalRobot) {
    if updated_robot.health == 0 {
      self.round_data.robots.remove(&updated_robot.id);
      self.game_data.robots.remove(&updated_robot.id);
      return;
    }

    if let Some(mut robot) = self.round_data.robots.get_mut(&updated_robot.id) {
      if let Some(robot_info) = self.game_data.robots.get_mut(&updated_robot.id) {
        if updated_robot.damage_level != robot.damage_level {
          robot_info.attack_damage = updated_robot.damage_level.get_attack_damage_value_for_level();
        }
        if updated_robot.energy_level != robot.energy_level {
          robot_info.max_energy = updated_robot.energy_level.get_max_energy_value_for_level();
        }
        if updated_robot.health_level != robot.health_level {
          robot_info.max_health = updated_robot.health_level.get_max_health_value_for_level();
        }
        if updated_robot.storage_level != robot.storage_level {
          robot_info.inventory.max_storage = updated_robot.storage_level.get_storage_value_for_level();
        }
        if updated_robot.mining_speed_level != robot.mining_speed_level {
          robot_info.mining_speed = updated_robot.mining_speed_level.get_mining_speed_value_for_level();
        }
        if updated_robot.energy_regen_level != robot.energy_regen_level {
          robot_info.energy_regen = updated_robot.energy_regen_level.get_energy_regen_value_for_level();
        }
      }

      robot = updated_robot;
    }
  }

  pub fn update_enemy_robot(&mut self, updated_robot: &mut MinimalRobot) {
    if updated_robot.health == 0 {
      self.round_data.enemy_robots.remove(&updated_robot.id);
      return;
    }

    if let Some(mut robot) = self.round_data.enemy_robots.get_mut(&updated_robot.id) {
      robot = updated_robot;
    }
  }

  pub fn update_inventory_add(&mut self, robot_id: String, mined_amount: u16, coal: u16, gem: u16, gold: u16, iron: u16, platin: u16) {
    if let Some(robot) = self.game_data.robots.get_mut(&robot_id) {
      robot.inventory.used_storage += mined_amount;

      if robot.inventory.max_storage <= robot.inventory.used_storage {
        robot.inventory.full = true;
        robot.inventory.used_storage = robot.inventory.max_storage;
      }

      robot.inventory.coal = coal;
      robot.inventory.gem = gem;
      robot.inventory.gold = gold;
      robot.inventory.iron = iron;
      robot.inventory.platin = platin;
    }
  }

  pub fn update_inventory_remove(&mut self, robot_id: String, removed_amount: u16, coal: u16, gem: u16, gold: u16, iron: u16, platin: u16) {
    if let Some(robot) = self.game_data.robots.get_mut(&robot_id) {
      robot.inventory.used_storage -= removed_amount;
      robot.inventory.full = false;

      robot.inventory.coal = coal;
      robot.inventory.gem = gem;
      robot.inventory.gold = gold;
      robot.inventory.iron = iron;
      robot.inventory.platin = platin;
    }
  }

  pub fn update_planet(&mut self, planet_id: String, mined_amount: u32) {
    if let Some(planet) = self.round_data.planets.get_mut(&planet_id) {
      planet.current_amount -= mined_amount;

      if let Some(planet_info) = self.game_data.planets.get_mut(&planet_id) {
        planet_info.last_known_amount = planet.current_amount;
      }
    }
  }

  pub fn save_planet(&mut self, planet: Planet, planet_info: PermanentPlanetInfo) {
    self.round_data.planets.insert(planet.id.clone(), planet);
    self.game_data.planets.insert(planet_info.id.clone(), planet_info);
  }

  pub fn clear_game(&mut self) {
    let player_id = self.game_data.player_id.clone();

    self.game_data = GameData::new();
    self.game_data.player_id = player_id;

    self.round_data = RoundData::new();
  }
}

pub async fn execute_purchase_robots_command(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, amount: u16) {
  let buy_robot_command = Command::create_robot_purchase_command(player_id, amount);
  info!("====> Try to buy Robots!!!!!!!!!!!!!!.");
  let _ = game_service_rest_adapter.send_command(buy_robot_command).await;
}

#[async_trait]
pub trait Action: Send + Sync {
  fn get_weight(&self) -> f32;
  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String);
}

pub struct MovementAction {
  pub weight: f32,
  pub dir: Direction,
  pub current_planet: PermanentPlanetInfo,
}

impl MovementAction {
  fn new(weight: f32, dir: Direction, current_planet: PermanentPlanetInfo) -> Self {
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

    let command = Command::create_movement_command(player_id, robot_id, planet_id);
    game_service_rest_adapter.send_command(command).await;
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

#[async_trait]
impl Action for AttackAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      let command = Command::create_robot_attack_command(player_id, robot_id, self.target_robot.clone());
      info!("====> Trying to Attack!!!!!!!!!!!");
      game_service_rest_adapter.send_command(command).await;
  }
}

pub struct RegenerateAction {
  pub weight: f32,
}

impl RegenerateAction {
  fn new(weight: f32) -> Self {
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
      let command = Command::create_robot_regenerate_command(player_id, robot_id);
      info!("====> Trying to Regenerate!!!!!!!!!!!");
      game_service_rest_adapter.send_command(command).await;
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

#[async_trait]
impl Action for SellAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
    let command = Command::create_robot_sell_inventory_command(player_id, robot_id);
    info!("====> Trying to Sell Inventory!!!!!!!!!!!");
    game_service_rest_adapter.send_command(command).await;
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

#[async_trait]
impl Action for MineAction {
  fn get_weight(&self) -> f32 {
    return self.weight;
  }

  async fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
    let command = Command::create_robot_mine_command(player_id, robot_id, self.target_planet_id.clone());
    info!("====> Trying to Attack!!!!!!!!!!!");
    game_service_rest_adapter.send_command(command).await;
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
  fn new() -> Self {
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

//////////////////////////// Event Handling //////////////////////
pub struct RobotsRevealedEventHandler {
  game: Arc<Mutex<GameLogic>>, // this needs to be a pointer doesn't it?
}

impl RobotsRevealedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
      Self {
        game,
      }
  }
}

#[async_trait]
impl EventHandler<RobotsRevealedEvent> for RobotsRevealedEventHandler {
  async fn handle(&self, event: RobotsRevealedEvent) {
    let mut game_mut = self.game.lock().await;
    for r in event.robots.iter() {
      let mut robot = MinimalRobot::new(r.robot_id.to_string(), r.planet_id.to_string(), r.energy, r.health, r.levels.health_level, r.levels.damage_level, r.levels.mining_speed_level, r.levels.mining_level, r.levels.energy_level, r.levels.energy_regen_level, r.levels.storage_level);
      
      if game_mut.game_data.player_id.starts_with(&r.player_notion) {
        if let Some(_) = game_mut.round_data.robots.get(&r.robot_id) {
          game_mut.update_robot(&mut robot);
        }
      }
      else {
        if let Some(_) = game_mut.round_data.enemy_robots.get(&r.robot_id) {
          game_mut.update_enemy_robot(&mut robot);
        }
      }
    }
  }
}

pub struct RobotSpawnedEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl RobotSpawnedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
      Self {
        game,
      }
  }
}

#[async_trait]
impl EventHandler<RobotSpawnedEvent> for RobotSpawnedEventHandler {
  async fn handle(&self, event: RobotSpawnedEvent) {
      let r = event.robot;
      let robot_info = MinimalRobot::new(r.robot_id.to_string(), r.planet.planet_id.to_string(), r.robot_attributes.energy, r.robot_attributes.health, r.robot_levels.health_level, r.robot_levels.damage_level, r.robot_levels.mining_speed_level, r.robot_levels.mining_level, r.robot_levels.energy_level, r.robot_levels.energy_regen_level, r.inventory.storage_level);

      let r_i = r.inventory.resources;
      let inventory = Inventory::new(r_i.coal, r_i.iron, r_i.gold, r_i.gem, r_i.platin, r.inventory.full, r.inventory.used_storage, r.inventory.max_storage);

      let robot = Robot::new(robot_info, inventory, r.robot_attributes.max_health, r.robot_attributes.max_energy, r.robot_attributes.energy_regen, r.robot_attributes.attack_damage, r.robot_attributes.mining_speed, r.player_id);

      self.game.lock().await.save_robot(robot);
  }
}

pub struct ResourceMinedEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl ResourceMinedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}
#[async_trait]
impl EventHandler<PlanetResourceMinedEvent> for ResourceMinedEventHandler {
  async fn handle(&self, event: PlanetResourceMinedEvent) {
    self.game.lock().await.update_planet(event.planet_id, event.mined_amount);
  }
}

pub struct PlanetDiscoveredEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl PlanetDiscoveredEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<PlanetDiscoveredEvent> for PlanetDiscoveredEventHandler {
  async fn handle(&self, event: PlanetDiscoveredEvent) {
    let planet = Planet::new(event.planet_id.clone(), event.resource.current_amount);

    let mut north_planet = String::new();
    let mut east_planet = String::new();
    let mut south_planet = String::new();
    let mut west_planet = String::new();


    for neighbour in event.neighbours {
      match neighbour.compass_direction {
        CompassDirection::NORTH => north_planet = neighbour.planet_id,
        CompassDirection::EAST => east_planet = neighbour.planet_id,
        CompassDirection::SOUTH => south_planet = neighbour.planet_id,
        CompassDirection::WEST => west_planet = neighbour.planet_id,
      }
    }

    let planet_info = PermanentPlanetInfo::new(event.planet_id.clone(), event.movement_difficulty, event.resource.resource_type, event.resource.max_amount, event.resource.current_amount, north_planet, east_planet, south_planet, west_planet);
    
    self.game.lock().await.save_planet(planet, planet_info);
  }
}

pub struct RobotResourceMinedEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl RobotResourceMinedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<RobotResourceMinedEvent> for RobotResourceMinedEventHandler {
  async fn handle(&self, event: RobotResourceMinedEvent) {
    self.game.lock().await.update_inventory_add(event.robot_id, event.mined_amount, event.resource_inventory.coal, event.resource_inventory.gem, event.resource_inventory.gold, event.resource_inventory.iron, event.resource_inventory.platin);
  }
}

pub struct RobotResourceRemovedEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl RobotResourceRemovedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<RobotResourceRemovedEvent> for RobotResourceRemovedEventHandler {
  async fn handle(&self, event: RobotResourceRemovedEvent) {
    self.game.lock().await.update_inventory_remove(event.robot_id, event.removed_amount, event.resource_inventory.coal, event.resource_inventory.gem, event.resource_inventory.gold, event.resource_inventory.iron, event.resource_inventory.platin);
  }
}

pub struct BankAccountInitializedEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl BankAccountInitializedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<BankAccountInitializedEvent> for BankAccountInitializedEventHandler {
  async fn handle(&self, event: BankAccountInitializedEvent) {
    let mut game_mut = self.game.lock().await;
    if event.player_id == game_mut.game_data.player_id {
      game_mut.balance_update(event.balance);
    }
  }
}

pub struct BankAccountTransactionBookedEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl BankAccountTransactionBookedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<BankAccountTransactionBookedEvent> for BankAccountTransactionBookedEventHandler {
  async fn handle(&self, event: BankAccountTransactionBookedEvent) {
    let mut game_mut = self.game.lock().await;
    if event.player_id == game_mut.game_data.player_id {
      game_mut.balance_update(event.balance);
    }
  }
}

pub struct TradablePricesEventHandler {
  game: Arc<Mutex<GameLogic>>,
}

impl TradablePricesEventHandler {
  pub fn new(game: Arc<Mutex<GameLogic>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<TradablePricesEvent> for TradablePricesEventHandler {
  async fn handle(&self, event: TradablePricesEvent) {
    let mut game_mut = self.game.lock().await;
    for item in event.items {
      match item.name.as_str() {
        "MINING_SPEED_1" => game_mut.update_item_price(TradeItemType::MiningSpeed1, item.price as f32),
        "MINING_SPEED_2" => game_mut.update_item_price(TradeItemType::MiningSpeed2, item.price as f32),
        "MINING_SPEED_3" => game_mut.update_item_price(TradeItemType::MiningSpeed3, item.price as f32),
        "MINING_SPEED_4" => game_mut.update_item_price(TradeItemType::MiningSpeed4, item.price as f32),
        "MINING_SPEED_5" => game_mut.update_item_price(TradeItemType::MiningSpeed5, item.price as f32),
        "MAX_ENERGY_1" => game_mut.update_item_price(TradeItemType::MaxEnergy1, item.price as f32),
        "MAX_ENERGY_2" => game_mut.update_item_price(TradeItemType::MaxEnergy2, item.price as f32),
        "MAX_ENERGY_3" => game_mut.update_item_price(TradeItemType::MaxEnergy3, item.price as f32),
        "MAX_ENERGY_4" => game_mut.update_item_price(TradeItemType::MaxEnergy4, item.price as f32),
        "MAX_ENERGY_5" => game_mut.update_item_price(TradeItemType::MaxEnergy5, item.price as f32),
        "ENERGY_REGEN_1" => game_mut.update_item_price(TradeItemType::EnergyRegen1, item.price as f32),
        "ENERGY_REGEN_2" => game_mut.update_item_price(TradeItemType::EnergyRegen2, item.price as f32),
        "ENERGY_REGEN_3" => game_mut.update_item_price(TradeItemType::EnergyRegen3, item.price as f32),
        "ENERGY_REGEN_4" => game_mut.update_item_price(TradeItemType::EnergyRegen4, item.price as f32),
        "ENERGY_REGEN_5" => game_mut.update_item_price(TradeItemType::EnergyRegen5, item.price as f32),
        "STORAGE_1" => game_mut.update_item_price(TradeItemType::Storage1, item.price as f32),
        "STORAGE_2" => game_mut.update_item_price(TradeItemType::Storage2, item.price as f32),
        "STORAGE_3" => game_mut.update_item_price(TradeItemType::Storage3, item.price as f32),
        "STORAGE_4" => game_mut.update_item_price(TradeItemType::Storage4, item.price as f32),
        "STORAGE_5" => game_mut.update_item_price(TradeItemType::Storage5, item.price as f32),
        "MINING_1" => game_mut.update_item_price(TradeItemType::Mining1, item.price as f32),
        "MINING_2" => game_mut.update_item_price(TradeItemType::Mining2, item.price as f32),
        "MINING_3" => game_mut.update_item_price(TradeItemType::Mining3, item.price as f32),
        "MINING_4" => game_mut.update_item_price(TradeItemType::Mining4, item.price as f32),
        "MINING_5" => game_mut.update_item_price(TradeItemType::Mining5, item.price as f32),
        "HEALTH_1" => game_mut.update_item_price(TradeItemType::Health1, item.price as f32),
        "HEALTH_2" => game_mut.update_item_price(TradeItemType::Health2, item.price as f32),
        "HEALTH_3" => game_mut.update_item_price(TradeItemType::Health3, item.price as f32),
        "HEALTH_4" => game_mut.update_item_price(TradeItemType::Health4, item.price as f32),
        "HEALTH_5" => game_mut.update_item_price(TradeItemType::Health5, item.price as f32),
        "DAMAGE_1" => game_mut.update_item_price(TradeItemType::Damage1, item.price as f32),
        "DAMAGE_2" => game_mut.update_item_price(TradeItemType::Damage2, item.price as f32),
        "DAMAGE_3" => game_mut.update_item_price(TradeItemType::Damage3, item.price as f32),
        "DAMAGE_4" => game_mut.update_item_price(TradeItemType::Damage4, item.price as f32),
        "DAMAGE_5" => game_mut.update_item_price(TradeItemType::Damage5, item.price as f32),
        "ENERGY_RESTORE" => game_mut.update_item_price(TradeItemType::EnergyRestore, item.price as f32),
        "HEALTH_RESTORE" => game_mut.update_item_price(TradeItemType::HealthRestore, item.price as f32),
        "ROBOT" => game_mut.update_item_price(TradeItemType::Robot, item.price as f32),
        "COAL" => game_mut.update_resource_price(MineableResourceType::COAL, item.price as f32),
        "IRON" => game_mut.update_resource_price(MineableResourceType::IRON, item.price as f32),
        "GEM" => game_mut.update_resource_price(MineableResourceType::GEM, item.price as f32),
        "GOLD" => game_mut.update_resource_price(MineableResourceType::GOLD, item.price as f32),
        "PLATIN" => game_mut.update_resource_price(MineableResourceType::PLATIN, item.price as f32),
        _ => {},
      }
    }
  }
}
