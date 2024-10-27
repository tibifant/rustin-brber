use std::collections::HashMap;

use crate::{domainprimitives::{location::mineable_resource_type::MineableResourceType, purchasing::trade_item_type::TradeItemType}, eventinfrastructure::robot::{self, dto::robot_resource_inventory_dto}, planet::domain::planet::{PersistentPlanetInfo, TransientPlanetInfo}, robot::domain::robot::{PersistentRobotInfo, RobotDecisionInfo, TransientRobotInfo}};

pub struct GameDecisionInfo {
  pub robots: HashMap<String, RobotDecisionInfo>
}

impl GameDecisionInfo {
  pub fn new() -> Self {
    let robots = HashMap::new();
    Self {
      robots
    }
  }
}

pub struct TransientData {
  pub robots: HashMap<String, TransientRobotInfo>,
  pub enemy_robots: HashMap<String, TransientRobotInfo>,
  pub planets: HashMap<String, TransientPlanetInfo>,
  pub balance: f32,
  pub item_prices: HashMap<TradeItemType, f32>,
  pub resource_prices: HashMap<MineableResourceType, f32>,
}

impl TransientData {
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

pub struct PersistentData {
  pub planets: HashMap<String, PersistentPlanetInfo>,
  pub robots: HashMap<String, PersistentRobotInfo>,
  pub player_id: String,
  pub robot_buy_amount: u16,
}

impl PersistentData {
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
