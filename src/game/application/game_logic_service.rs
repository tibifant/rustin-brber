use std::sync::Arc;

use crate::domainprimitives::command::action::{execute_purchase_robots_command, Action, AttackAction, MineAction, MovementAction, NoneAction, PurchaseAction, RegenerateAction, SellAction};
use crate::domainprimitives::location::direction::Direction;
use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;
use crate::domainprimitives::purchasing::robot_level::RobotLevel;
use crate::domainprimitives::purchasing::robot_upgrade_type::RobotUpgradeType;
use crate::domainprimitives::purchasing::trade_item_type::TradeItemType;
use crate::eventinfrastructure::robot;
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
        self.offer_sell_option(id.to_string(), r);
      }
    }
    
    while self.round_data.balance > 0. {
      //if !self.spend_money(&mut decision_info) {
      //  break;

      // we generated a wild error when sending the purchase item command - I really need to go to sleep and don't have any patience for fixing the skeleton left. So only robot purchases allowed.
      if self.round_data.balance >= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&99999.) {
        self.game_data.robot_buy_amount += 1;
        self.round_data.balance -= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&99999.);
      }
      else {
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
    let mut set_action = false;
    if let Some(robot_info) = self.game_data.robots.get(&robot_id) {
      if let Some(robot) = self.round_data.robots.get(&robot_id) {
        if robot.energy > 3 { // not sure how much energy we need for which action
          if let Some(planet) = self.game_data.planets.get(&robot.planet_id) {
            let mut known_neighbours = 0;
            if let Some(resource) = planet.resource {
              for (e_id, e) in &self.round_data.enemy_robots {
                if e.planet_id == robot.planet_id {
                  let weight: f32 = (robot_info.attack_damage - e.damage_level.get_attack_damage_value_for_level()) as f32;
                
                  if robot_decision.action.get_weight() < weight {
                    let attack_option: Box<dyn Action + Send + Sync> = Box::new(AttackAction::new(weight, e_id.to_string()));
                    robot_decision.action = attack_option;
                    set_action = true;
                  }
                  break;
                }
              }
              
              let mut best_planet = Direction::Here;
              let mut best_price = *self.round_data.resource_prices.get(&resource.resource_type).unwrap_or(&0.);
              let mut best_planet_amount = resource.current_amount;

              if let Some(p) = self.game_data.planets.get(&planet.north) {
                self.evaluate_planet(robot.mining_level, Direction::North, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if let Some(p) = self.game_data.planets.get(&planet.south) {
                self.evaluate_planet(robot.mining_level, Direction::South, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if let Some(p) = self.game_data.planets.get(&planet.west) {
                self.evaluate_planet(robot.mining_level, Direction::West, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if let Some(p) = self.game_data.planets.get(&planet.east) {
                self.evaluate_planet(robot.mining_level, Direction::East, p, &mut best_planet, &mut best_price, &mut best_planet_amount);
                known_neighbours += 1;
              }
            
              if best_planet_amount as f32 * best_price > 0. {
                if best_planet != Direction::Here {
                  let weight = best_price + best_planet_amount as f32;
                
                  if robot_decision.action.get_weight() < weight {
                    let movement_option: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(weight, best_planet, planet.clone()));
                    robot_decision.action = movement_option;
                    set_action = true;
                  }
                } else if !robot_info.inventory.full {
                  let weight = resource.current_amount /* LAST KNOWN, not *ACTUALLY* CURRENT */ as f32 + best_price;
                
                  if robot_decision.action.get_weight() < weight && (robot.mining_level as u8) >= (resource.resource_type as u8) {
                    let mining_option: Box<dyn Action + Send + Sync> = Box::new(MineAction::new(weight, planet.id.to_string()));
                    robot_decision.action = mining_option;
                    set_action = true;
                  }
                }
              }
              else {
                if robot_decision.action.get_weight() < 20000. {
                  let a: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(20000., Direction::East, planet.clone()));
                  robot_decision.action = a;
                  set_action = true;
                }
              }
            }
            else {
              if robot_decision.action.get_weight() < 20000. {
                let a: Box<dyn Action + Send + Sync> = Box::new(MovementAction::new(20000., Direction::East, planet.clone()));
                robot_decision.action = a;
                set_action = true;
              }
            }

            if known_neighbours < 4 && robot_info.move_count < 4 {
              match robot_info.move_count {
                0 => { 
                    if planet.north != "" {
                      robot_decision.action = Box::new(MovementAction::new(9999999., Direction::North, planet.clone()));
                      set_action = true;
                    }
                  }
                1 =>  {
                    if planet.east != "" {
                      robot_decision.action = Box::new(MovementAction::new(9999999., Direction::East, planet.clone()));
                      set_action = true;
                    }
                  }
                2 | 3 =>  {
                    if planet.south != "" { 
                      robot_decision.action = Box::new(MovementAction::new(9999999., Direction::South, planet.clone()));
                      set_action = true;
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
          set_action = true;
        }

        if !set_action {
          let a: Box<dyn Action + Send + Sync> = Box::new(RegenerateAction::new(9999.));
          robot_decision.action = a;
        }
      }
    }
  }

  fn evaluate_planet(&self, mining_level: RobotLevel, dir: Direction, planet: &PersistentPlanetInfo, best_planet: &mut Direction, best_price: &mut f32, best_amount: &mut u32) {
    if let Some(rsrc) = planet.resource {
      if (mining_level as u8) < (rsrc.resource_type as u8) {
        return;
      }
    }
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
          let inventory_weight = robot_info.inventory.used_storage as f32 * 1000.;
          
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
    let mut lowest_level_robot = String::new();
    let mut lowest_robot_level : RobotLevel = RobotLevel::LEVEL5;
    let mut highest_robot_level : RobotLevel = RobotLevel::LEVEL0;
    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    let robot_count = ids.len();

    for id in ids {
      if let Some(robot) = self.round_data.robots.get(&id) {
        if let Some(robot_decision_info) = decision_info.robots.get_mut(&id) {
          if (robot_decision_info.has_upgrade) {
            continue;
          }
        }

        if ((robot.mining_level as u16) <= (lowest_robot_level as u16)) {
          lowest_level_robot = id;
          lowest_robot_level = robot.mining_level;
        }

        if ((robot.mining_level as u16) > (highest_robot_level as u16)) {
          highest_robot_level = robot.mining_level;
        }
      }
    }

    let min_robot_count = ((highest_robot_level as usize) + 1) * 3;

    if lowest_robot_level == highest_robot_level && highest_robot_level != RobotLevel::LEVEL0 || robot_count < min_robot_count || lowest_level_robot.is_empty() {
      if self.round_data.balance >= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&99999.) {
        self.game_data.robot_buy_amount += 1;
        self.round_data.balance -= *self.round_data.item_prices.get(&TradeItemType::Robot).unwrap_or(&99999.);
        return true;
      } else {
        return false;
      }
    } else {
      // buy upgrade for lowest_level_robot, if sufficient funds.
      if let Some(item) = TradeItemType::get_next_level_item(RobotUpgradeType::Mining, (lowest_robot_level as u16)) {
        if self.round_data.balance >= *self.round_data.item_prices.get(&item).unwrap_or(&99999.) {
          if let Some(r) = decision_info.robots.get_mut(&lowest_level_robot) {
            r.has_upgrade = true;
            r.upgrade_action = Box::new(PurchaseAction::new(10000., item));
            self.round_data.balance -= *self.round_data.item_prices.get(&item).unwrap_or(&99999.);
            return true;
          }
        } else {
          return false;
        }
      }
    }

    let ids: Vec<String> = self.game_data.robots.keys().cloned().collect();
    for id in ids {
      if let Some(robot_decision_info) = decision_info.robots.get_mut(&id) {
        if let Some(robot_info) = self.game_data.robots.get(&id) {
          if let Some(robot) = self.round_data.robots.get(&id) {
            if !robot_decision_info.has_upgrade {
              if let Some(health_price) = self.round_data.item_prices.get(&TradeItemType::HealthRestore) {
                if self.round_data.balance >= *health_price && robot.health < 1 {
                  let weight = 3000.;
                  let item = TradeItemType::HealthRestore;
                  
                  robot_decision_info.has_upgrade = true;
                  robot_decision_info.upgrade_action = Box::new(PurchaseAction::new(weight, item));
                  self.round_data.balance -= *self.round_data.item_prices.get(&item).unwrap_or(&99999.);
                  return true;
                }
              }
            }
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
  pub fn update_robot_level(&mut self, robot_id: String, level: RobotLevel, upgrade: RobotUpgradeType) {
    if let Some(r) = self.round_data.robots.get_mut(&robot_id) {
      match upgrade {
        RobotUpgradeType::Mining => r.mining_level = level,
        _ => (),
        // TODO add other upgrade types here, if used!
      }
    }
  }
  pub fn update_robot_energy(&mut self, robot_id: String, available_energy: u16) {
    if let Some(r) = self.round_data.robots.get_mut(&robot_id) {
      r.energy = available_energy
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
