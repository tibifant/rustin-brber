use std::f32::consts::E;
use std::{collections::HashMap, hash::Hash};

use std::sync::Arc;

use crate::domainprimitives::location::compass_direction_dto::CompassDirection;
use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::map::planet_discovered_event::PlanetDiscoveredEvent;
use crate::eventinfrastructure::map::planet_resource_mined_event::PlanetResourceMinedEvent;
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
  pub action: Arc<dyn Action>,
  pub regen: bool,
  pub upgrade_action: Arc<dyn Action>,
  pub upgrade: bool,
  pub max_health: u16,
  pub max_energy: u16,
  pub energy_regen: u16,
  pub attack_damage: u16,
  pub mining_speed: u16,
  pub inventory: Inventory,           
}

impl PermanetRobotInfo {
  pub fn new(id: String, player_id: String, action: Arc<dyn Action>, regen: bool, upgrade_action: Arc<dyn Action>, upgrade: bool, max_health: u16,max_energy: u16, energy_regen: u16, attack_damage: u16, mining_speed: u16, inventory: Inventory) -> Self {
    Self {
      id,
      player_id,
      action,
      regen,
      upgrade_action,
      upgrade,
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

pub struct GameData {
  pub planets: HashMap<String, PermanentPlanetInfo>,
  pub robots: HashMap<String, PermanetRobotInfo>,
  pub player_id: String,
  pub robot_buy_amount: u16,
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
  pub fn round_move(&mut self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>) {

    self.game_data.robot_buy_amount = 0;

    for (id, robot) in &mut self.game_data.robots {
      robot.action = Arc::new(NoneAction::new());
      robot.upgrade = false;
      robot.upgrade_action = Arc::new(NoneAction::new());
      robot.regen = false;
    }

    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    for id in ids {
      self.offer_movement_mining_attack_option(id.to_string());
      self.offer_sell_option(id.to_string());
    }
    
    while self.round_data.balance > 0. {
      if !self.spend_money() {
        break;
      }
    }

    for (id, robot) in &mut self.game_data.robots {
      robot.action.execute_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), robot.id.to_string()); // execute best option
    }

    execute_purchase_robots_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), self.game_data.robot_buy_amount);
  }

  fn offer_movement_mining_attack_option(&mut self, robot_id: String) {
    if let Some(robot_info) = self.game_data.robots.get_mut(&robot_id) {
      if let Some(robot) = self.round_data.robots.get(&robot_id) {
        let planet = self.game_data.planets.get(&robot.planet_id);

        match planet {
          Some(planet) => {
            for (e_id, e) in &self.round_data.enemy_robots {
              if e.planet_id == robot.planet_id {
                let weight: f32 = (robot_info.attack_damage - e.damage_level.get_attack_damage_value_for_level()) as f32;

                if robot_info.action.get_weight() < weight {
                  let attack_option = Arc::new(AttackAction::new(weight, e_id.to_string()));
                  robot_info.action = attack_option;
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

                  if robot_info.action.get_weight() < weight {
                    let movement_option = Arc::new(MovementAction::new(weight, best_planet));
                    robot_info.action = movement_option;
                  }
                } else {
                  let weight = planet.last_known_amount as f32 + best_price;

                  if robot_info.action.get_weight() < weight {
                    let mining_option = Arc::new(MineAction::new(weight, planet.id.to_string()));
                    robot_info.action = mining_option;
                  }
                }
              }
              else {
                if robot_info.action.get_weight() < 2000. {
                  robot_info.action = Arc::new(MovementAction::new(2000., Direction::East));
                }
              }
            }
          }
          None => {}
        }
      }
    }
  }

  fn offer_sell_option(&mut self, robot_id: String) {
    if let Some(robot_info) = self.game_data.robots.get_mut(&robot_id) {
      if let Some(robot) = self.round_data.robots.get(robot_info.id.as_str()) {
        let inventory_weight = 0.1 * ((robot_info.max_health - robot.health) as f32) * (robot_info.inventory.coal as f32) * self.round_data.resource_prices.get(&MineableResourceType::COAL).unwrap_or(&0.) + (robot_info.inventory.gem as f32) * self.round_data.resource_prices.get(&MineableResourceType::GEM).unwrap_or(&0.) + (robot_info.inventory.gold  as f32) * self.round_data.resource_prices.get(&MineableResourceType::GOLD).unwrap_or(&0.)+ (robot_info.inventory.iron as f32) * self.round_data.resource_prices.get(&MineableResourceType::IRON).unwrap_or(&0.) + (robot_info.inventory.platin as f32) * self.round_data.resource_prices.get(&MineableResourceType::PLATIN).unwrap_or(&0.);

        if inventory_weight > robot_info.action.get_weight() {
          let inventory_option = Arc::new(SellAction::new(inventory_weight as f32));
          robot_info.action = inventory_option;
        }
      }
    }
  }

  fn spend_money(&mut self) -> bool {
    let mut best_weight: f32 = 0.;
    let mut best_item = TradeItemType::Robot;
    let mut best_id: Option<String> = None;
    let mut is_upgrade = false;

    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    for id in ids {
      if let Some(robot_info) = self.game_data.robots.get(&id) {
        if let Some(robot) = self.round_data.robots.get(&id) {
          if !robot_info.regen {
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
          }

          if !robot_info.upgrade {
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
            // TODO implement MiningSpeed or something
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
        if let Some(best_robot) = self.game_data.robots.get_mut(&id) {
          if !is_upgrade {
            if best_weight > best_robot.action.get_weight() {
              best_robot.action = Arc::new(PurchaseAction::new(best_weight as f32, best_item));
              best_robot.regen = true;
              self.round_data.balance -= *self.round_data.item_prices.get(&(best_item.clone())).unwrap_or(&10000000.);
              return true;
            }
          }
          else {
            best_robot.upgrade_action = Arc::new(PurchaseAction::new(best_weight as f32, best_item));
            best_robot.upgrade = true;
            self.round_data.balance -= *self.round_data.item_prices.get(&(best_item.clone())).unwrap_or(&10000000.);
            return true;
          }
        }
      }
    }
    return false;
  }

  // Event Stuff
  // TODO: assign player_id
  // TODO: handle bank_acoutn_created
  // TODO every round:
  // resource_mined, resource_removed, transaction_booked, planet_revealed

  pub fn balance_update(&mut self, balance: f32) {
    self.round_data.balance = balance;
  }

  pub fn save_robot(&mut self, robot: Robot) {
    if robot.player_id == self.game_data.player_id {
      let new_robot_info = PermanetRobotInfo::new(robot.robot_info.id.clone(), robot.player_id, Arc::new(NoneAction::new()), false, Arc::new(NoneAction::new()), false, robot.max_health, robot.max_energy, robot.energy_regen, robot.attack_damage, robot.mining_speed, robot.inventory);
      
      self.game_data.robots.insert(new_robot_info.id.clone(), new_robot_info);      
      self.round_data.robots.insert(robot.robot_info.id.clone(), robot.robot_info);
    }
    else {
      self.round_data.enemy_robots.insert(robot.robot_info.id.clone(), robot.robot_info);
    }
  }

  pub fn update_robots(&mut self, updated_robot: &mut MinimalRobot) {
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

  pub fn update_enemy_robots(&mut self, updated_robot: &mut MinimalRobot) {
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
}

pub fn execute_purchase_robots_command(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, amount: u16) {
  println!(" ");
}

pub trait Action {
  fn get_weight(&self) -> f32;
  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String);
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

  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      
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

  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      
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

  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      
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

  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      
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

  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      
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

impl Action for NoneAction {
  fn get_weight(&self) -> f32 {
      return self.weight;
  }

  fn execute_command(&self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, player_id: String, robot_id: String) {
      return;
  }
}

//////////////////////////// Event Handling //////////////////////
pub struct RobotsRevealedEventHandler {
  game: GameLogic, // this needs to be a pointer doesn't it?
}

impl RobotsRevealedEventHandler {
  pub fn new(game: GameLogic) -> Self {
      Self {
        game,
      }
  }
}

impl EventHandler<RobotsRevealedEvent> for RobotsRevealedEventHandler {
  fn handle(&mut self, event: RobotsRevealedEvent) {
    for r in event.robots.iter() {
      let mut robot = MinimalRobot::new(r.robot_id.to_string(), r.planet_id.to_string(), r.energy, r.health, r.levels.health_level, r.levels.damage_level, r.levels.mining_speed_level, r.levels.mining_level, r.levels.energy_level, r.levels.energy_regen_level, r.levels.storage_level);
      
      if self.game.game_data.player_id.starts_with(&r.player_notion) {
        if let Some(_) = self.game.round_data.robots.get(&r.robot_id) {
          self.game.update_robots(&mut robot);
        }
      }
      else {
        if let Some(_) = self.game.round_data.enemy_robots.get(&r.robot_id) {
          self.game.update_enemy_robots(&mut robot);
        }
      }
    }
  }
}

pub struct RobotSpawnedEventHandler {
  game: GameLogic,
}

impl RobotSpawnedEventHandler {
  pub fn new(game: GameLogic) -> Self {
      Self {
          game,
      }
  }
}

impl EventHandler<RobotSpawnedEvent> for RobotSpawnedEventHandler {
  fn handle(&mut self, event: RobotSpawnedEvent) {
      let r = event.robot;
      let robot_info = MinimalRobot::new(r.robot_id.to_string(), r.planet.planet_id.to_string(), r.robot_attributes.energy, r.robot_attributes.health, r.robot_levels.health_level, r.robot_levels.damage_level, r.robot_levels.mining_speed_level, r.robot_levels.mining_level, r.robot_levels.energy_level, r.robot_levels.energy_regen_level, r.inventory.storage_level);

      let r_i = r.inventory.resources;
      let inventory = Inventory::new(r_i.coal, r_i.iron, r_i.gold, r_i.gem, r_i.platin, r.inventory.full, r.inventory.used_storage, r.inventory.max_storage);

      let robot = Robot::new(robot_info, inventory, r.robot_attributes.max_health, r.robot_attributes.max_energy, r.robot_attributes.energy_regen, r.robot_attributes.attack_damage, r.robot_attributes.mining_speed, r.player_id);

      self.game.save_robot(robot);
  }
}

pub struct ResourceMinedEventHandler {
  game: GameLogic,
}

impl ResourceMinedEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<PlanetResourceMinedEvent> for ResourceMinedEventHandler {
  fn handle(&mut self, event: PlanetResourceMinedEvent) {
    self.game.update_planet(event.planet_id, event.mined_amount);
  }
}

pub struct PlanetDiscoveredEventHandler {
  game: GameLogic,
}

impl PlanetDiscoveredEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<PlanetDiscoveredEvent> for PlanetDiscoveredEventHandler {
  fn handle(&mut self, event: PlanetDiscoveredEvent) {
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
    
    self.game.save_planet(planet, planet_info);
  }
}

pub struct RobotResourceMinedEventHandler {
  game: GameLogic,
}

impl RobotResourceMinedEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<RobotResourceMinedEvent> for RobotResourceMinedEventHandler {
  fn handle(&mut self, event: RobotResourceMinedEvent) {
    self.game.update_inventory_add(event.robot_id, event.mined_amount, event.resource_inventory.coal, event.resource_inventory.gem, event.resource_inventory.gold, event.resource_inventory.iron, event.resource_inventory.platin);
  }
}

pub struct RobotResourceRemovedEventHandler {
  game: GameLogic,
}

impl RobotResourceRemovedEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<RobotResourceRemovedEvent> for RobotResourceRemovedEventHandler {
  fn handle(&mut self, event: RobotResourceRemovedEvent) {
    self.game.update_inventory_add(event.robot_id, event.removed_amount, event.resource_inventory.coal, event.resource_inventory.gem, event.resource_inventory.gold, event.resource_inventory.iron, event.resource_inventory.platin);
  }
}

pub struct BankAccountInitializedEventHandler {
  game: GameLogic,
}

impl BankAccountInitializedEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<BankAccountInitializedEvent> for BankAccountInitializedEventHandler {
  fn handle(&mut self, event: BankAccountInitializedEvent) {
    if event.player_id == self.game.game_data.player_id {
      self.game.balance_update(event.balance);
    }
  }
}

pub struct BankAccountTransactionBookedEventHandler {
  game: GameLogic,
}

impl BankAccountTransactionBookedEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<BankAccountTransactionBookedEvent> for BankAccountTransactionBookedEventHandler {
  fn handle(&mut self, event: BankAccountTransactionBookedEvent) {
    if event.player_id == self.game.game_data.player_id {
      self.game.balance_update(event.balance);
    }
  }
}

pub struct TradablePricesEventHandler {
  game: GameLogic,
}

impl TradablePricesEventHandler {
  pub fn new(game: GameLogic) -> Self {
    Self {
      game,
    }
  }
}

impl EventHandler<TradablePricesEvent> for TradablePricesEventHandler {
  fn handle(&mut self, event: TradablePricesEvent) {
    
  }
}
