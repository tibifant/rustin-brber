use std::sync::Arc;

use crate::domainprimitives::command::action::{execute_purchase_robots_command, Action, AttackAction, MineAction, MovementAction, NoneAction, PurchaseAction, RegenerateAction, SellAction};
use crate::domainprimitives::location::direction::Direction;
use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;
use crate::domainprimitives::purchasing::robot_upgrade_type::RobotUpgradeType;
use crate::domainprimitives::purchasing::trade_item_type::TradeItemType;
use crate::game::domain::game_logic_info::{GameDecisionInfo, PersistentData, TransientData};
use crate::planet::domain::planet::{PersistentPlanetInfo, TransientPlanetInfo};
use crate::rest::game_service_rest_adapter_trait::{self, GameServiceRestAdapterTrait};
use crate::robot::domain::robot::{PersistentRobotInfo, Robot, RobotDecisionInfo, TransientRobotInfo};

pub struct GameLogicService {
  pub round_data: TransientData,
  pub game_data: PersistentData,
}

impl GameLogicService {
  pub fn new() -> Self {
    let transient_data = TransientData::new();
    let persistent_data = PersistentData::new();

    Self {
      round_data: transient_data,
      game_data: persistent_data,
    }
  }

  pub async fn round_move(&mut self, game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>) {

    self.game_data.robot_buy_amount = 0;

    let mut decision_info = GameDecisionInfo::new();

    for (id, robot) in &mut self.game_data.robots {
      let r = RobotDecisionInfo::new(id.clone(), Box::new(NoneAction::new()), Box::new(NoneAction::new()), false);
      decision_info.robots.insert(id.clone(), r);
    }

    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    for id in ids {
      if let Some(r) = decision_info.robots.get_mut(&id) {
        self.offer_movement_mining_attack_option(id.to_string(), r);
        self.offer_sell_option(id.to_string(), r); // todo
      }
    }
    
    while self.round_data.balance > 0. {
      if !self.spend_money(&mut decision_info) {
        break;
      }
    }

    for (id, robot) in &mut decision_info.robots {
      if let Some(robot) = self.game_data.robots.get_mut(id) {
        robot.move_count += 1;
      }

      robot.action.execute_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), robot.id.to_string()).await;
      robot.upgrade_action.execute_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), robot.id.to_string()).await;
    }

    if self.game_data.robot_buy_amount > 0 {
      execute_purchase_robots_command(game_service_rest_adapter.clone(), self.game_data.player_id.to_string(), self.game_data.robot_buy_amount).await;
    }
  }

  fn offer_movement_mining_attack_option(&mut self, robot_id: String, robot_decision: &mut RobotDecisionInfo) {
    if let Some(robot_info) = self.game_data.robots.get(&robot_id) {
      if let Some(robot) = self.round_data.robots.get(&robot_id) {
        if robot.energy > 0 {
          if let Some(planet) = self.game_data.planets.get(&robot.planet_id) {
            let mut known_neighbours = 0;
            if let Some(resource) = planet.resource {
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
              let mut best_price = *self.round_data.resource_prices.get(&resource.resource_type).unwrap_or(&0.);
              let mut best_planet_amount = resource.current_amount;

              if let Some(p) = self.game_data.planets.get(&planet.north) {
                self.evaluate_planet(Direction::North, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if let Some(p) = self.game_data.planets.get(&planet.south) {
                self.evaluate_planet(Direction::South, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if let Some(p) = self.game_data.planets.get(&planet.west) {
                self.evaluate_planet(Direction::West, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if let Some(p) = self.game_data.planets.get(&planet.east) {
                self.evaluate_planet(Direction::East, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if best_planet_amount as f32 * best_price > 0. {
                if best_planet != Direction::Here {
                  let weight = best_price + best_planet_amount as f32;
                
                  if robot_decision.action.get_weight() < weight {
                    let movement_option: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(weight, best_planet, planet.clone()));
                    robot_decision.action = movement_option;
                  }
                } else if !robot_info.inventory.full {
                  let weight = resource.current_amount /* LAST KNOWN, not *ACTUALLY* CURRENT */ as f32 + best_price;
                
                  if robot_decision.action.get_weight() < weight {
                    let mining_option: Box<dyn Action + Send + Sync> = Box::new(MineAction::new(weight, planet.id.to_string()));
                    robot_decision.action = mining_option;
                  }
                }
              }
              else {
                if robot_decision.action.get_weight() < 20000. {
                  let a: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(20000., Direction::East, planet.clone()));
                  robot_decision.action = a;
                }
              }
            }
            else {
              if robot_decision.action.get_weight() < 20000. {
                let a: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(20000., Direction::East, planet.clone()));
                robot_decision.action = a;
              }
            }

            if known_neighbours < 4 && robot_info.move_count < 4 {
              match robot_info.move_count {
                0 => { 
                    if planet.north != "" {
                      robot_decision.action = Box::new(MovementAction::new(9999999., Direction::North, planet.clone()));
                    }
                  }
                1 =>  {
                    if planet.east != "" {
                      robot_decision.action = Box::new(MovementAction::new(9999999., Direction::East, planet.clone()));
                    }
                  }
                2 | 3 =>  {
                    if planet.south != "" { 
                      robot_decision.action = Box::new(MovementAction::new(9999999., Direction::South, planet.clone()));
                    }
                  }
                _ => (),
              }
            }

          }
        }
        else {
          let a: Box<dyn Action + Send + Sync> = Box::new(RegenerateAction::new(9999999.));
          robot_decision.action = a;
        }
      }
    }
  }

  fn evaluate_planet(&self, dir: Direction, planet: &PersistentPlanetInfo, best_planet: &mut Direction, best_price: &mut f32, best_amount: &mut u32) {
    if let Some(resource) = planet.resource {
      let price = self.round_data.resource_prices.get(&resource.resource_type).unwrap_or(&0.);
      if (price > best_price) {
        *best_price = *price;
        *best_planet = dir;
        *best_amount = resource.current_amount; // LAST KNOWN, not *ACTUALLY* CURRENT
      }
    }
  }

  fn offer_sell_option(&mut self, robot_id: String, robot_decision: &mut RobotDecisionInfo) {
    if let Some(robot_info) = self.game_data.robots.get(&robot_id) {
      if !robot_info.inventory.full {
          let inventory_weight = robot_info.inventory.used_storage as f32 * 100.;
          
          if inventory_weight > robot_decision.action.get_weight() {
            let inventory_option: Box<dyn Action + Send + Sync> = Box::new(SellAction::new(inventory_weight as f32));
            robot_decision.action = inventory_option;
          }
      }
      else {
        if 200000. > robot_decision.action.get_weight() {
          robot_decision.action = Box::new(SellAction::new(200000.));
        }
      }
    }
  }

  fn spend_money(&mut self, decision_info: &mut GameDecisionInfo) -> bool {
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
                        if let Some(resource) = planet.resource {
                          if let Some(resource_price) = self.round_data.resource_prices.get(&resource.resource_type) {
                            let weight = (resource.current_amount as f32) * resource_price + robot_info.inventory.  used_storage as f32; // LAST KNOWN, not *ACTUALLY* CURRENT
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
      let new_robot_info = PersistentRobotInfo::new(robot.robot_info.id.clone(), robot.player_id, robot.max_health, robot.max_energy, robot.energy_regen, robot.attack_damage, robot.mining_speed, robot.inventory);
      
      self.game_data.robots.insert(new_robot_info.id.clone(), new_robot_info);      
      self.round_data.robots.insert(robot.robot_info.id.clone(), robot.robot_info);
    }
    else {
      self.round_data.enemy_robots.insert(robot.robot_info.id.clone(), robot.robot_info);
    }
  }

  pub fn update_robot(&mut self, updated_robot: &mut TransientRobotInfo) {
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

  pub fn update_enemy_robot(&mut self, updated_robot: &mut TransientRobotInfo) {
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

  pub fn update_robot_location(&mut self, robot_id: String, new_planet: String, remaining_energy: u16) {
    if let Some(r) = self.round_data.robots.get_mut(&robot_id) {
      r.planet_id = new_planet;
      r.energy = remaining_energy;
    }
    else {
      if let Some(r) = self.round_data.robots.get_mut(&robot_id) {
        r.planet_id = new_planet;
        r.energy = remaining_energy;
      }
    }
  }

  pub fn update_planet(&mut self, planet_id: String, mined_amount: u32) {
    if let Some(planet) = self.round_data.planets.get_mut(&planet_id) {
      if let Some(mut r) = planet.resource {
        r.current_amount -= mined_amount;
      
        if let Some(planet_info) = self.game_data.planets.get_mut(&planet_id) {
          if let Some(mut resource) = planet_info.resource {
            resource.current_amount = r.current_amount;
          }
        }
      }
    }
  }

  pub fn save_planet(&mut self, planet: TransientPlanetInfo, planet_info: PersistentPlanetInfo) {
    self.round_data.planets.insert(planet.id.clone(), planet.clone());
    self.game_data.planets.insert(planet_info.id.clone(), planet_info.clone());

    print!("\n\n\nsaving planet ({}) with neigbours: \n north: {}\n east: {}\n south: {}\n west: {}\n\n\n", planet.id, planet_info.north, planet_info.east, planet_info.south, planet_info.west)
  }

  pub fn clear_game(&mut self) {
    let player_id = self.game_data.player_id.clone();

    self.game_data = PersistentData::new();
    self.game_data.player_id = player_id;

    self.round_data = TransientData::new();
  }
}
