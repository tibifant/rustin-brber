use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{domainprimitives::location::compass_direction_dto::CompassDirection, eventinfrastructure::{event_handler::EventHandler, map::{planet_discovered_event::PlanetDiscoveredEvent, planet_resource_mined_event::PlanetResourceMinedEvent}}, game::application::game_logic_service::GameLogicService, planet::domain::planet::{PersistentPlanetInfo, TransientPlanetInfo}};

pub struct ResourceMinedEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl ResourceMinedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}
#[async_trait]
impl EventHandler<PlanetResourceMinedEvent> for ResourceMinedEventHandler {
  async fn handle(&self, event: PlanetResourceMinedEvent) {
    self.game.lock().await.update_planet(event.planet, event.mined_amount);
  }
}

pub struct PlanetDiscoveredEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl PlanetDiscoveredEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<PlanetDiscoveredEvent> for PlanetDiscoveredEventHandler {
  async fn handle(&self, event: PlanetDiscoveredEvent) {
    
      let planet = TransientPlanetInfo::new(event.planet.clone(), event.resource);

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

      let planet_info = PersistentPlanetInfo::new(event.planet.clone(), event.movement_difficulty, event.resource, north_planet, east_planet, south_planet, west_planet);

      self.game.lock().await.save_planet(planet, planet_info);
  }
}
