use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{eventinfrastructure::{event_handler::EventHandler, robot::{robot_moved_event::RobotMovedEvent, robot_resource_mined_event::RobotResourceMinedEvent, robot_resource_removed_event::RobotResourceRemovedEvent, robot_spawned_event::RobotSpawnedEvent, robot_upgraded_event::RobotUpgradedEvent, robots_revealed_event::RobotsRevealedEvent}}, game::application::game_logic_service::GameLogicService, robot::domain::robot::{Inventory, Robot, TransientRobotInfo}};

pub struct RobotsRevealedEventHandler {
  game: Arc<Mutex<GameLogicService>>, // this needs to be a pointer doesn't it?
}

impl RobotsRevealedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
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
      let mut robot = TransientRobotInfo::new(r.robot_id.to_string(), r.planet_id.to_string(), r.energy, r.health, r.levels.health_level, r.levels.damage_level, r.levels.mining_speed_level, r.levels.mining_level, r.levels.energy_level, r.levels.energy_regen_level, r.levels.storage_level);
      
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
  game: Arc<Mutex<GameLogicService>>,
}

impl RobotSpawnedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
      Self {
        game,
      }
  }
}

#[async_trait]
impl EventHandler<RobotSpawnedEvent> for RobotSpawnedEventHandler {
  async fn handle(&self, event: RobotSpawnedEvent) {
      let r = event.robot;
      let robot_info = TransientRobotInfo::new(r.robot_id.to_string(), r.planet.planet_id.to_string(), r.robot_attributes.energy, r.robot_attributes.health, r.robot_levels.health_level, r.robot_levels.damage_level, r.robot_levels.mining_speed_level, r.robot_levels.mining_level, r.robot_levels.energy_level, r.robot_levels.energy_regen_level, r.inventory.storage_level);

      let r_i = r.inventory.resources;
      let inventory = Inventory::new(r_i.coal, r_i.iron, r_i.gold, r_i.gem, r_i.platin, r.inventory.full, r.inventory.used_storage, r.inventory.max_storage);

      let robot = Robot::new(robot_info, inventory, r.robot_attributes.max_health, r.robot_attributes.max_energy, r.robot_attributes.energy_regen, r.robot_attributes.attack_damage, r.robot_attributes.mining_speed, r.player_id);

      self.game.lock().await.save_robot(robot);
  }
}

pub struct RobotResourceMinedEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl RobotResourceMinedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
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
  game: Arc<Mutex<GameLogicService>>,
}

impl RobotResourceRemovedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
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

pub struct RobotMovedEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl RobotMovedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<RobotMovedEvent> for RobotMovedEventHandler {
  async fn handle(&self, event: RobotMovedEvent) {
    self.game.lock().await.update_robot_location(event.robot_id, event.to_planet.planet_id, event.remaining_energy);
  }
}

pub struct RobotUpgradedEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl RobotUpgradedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<RobotUpgradedEvent> for RobotUpgradedEventHandler {
  async fn handle(&self, event: RobotUpgradedEvent) {
    self.game.lock().await.update_robot_level(event.robot_id, event.level, event.upgrade);
  }
}
