use std::sync::Arc;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game_event::GameEvent;
use crate::eventinfrastructure::game_event_body_type::GameEventBodyType;
use crate::game::application::game_application_service::GameApplicationService;
use crate::game::application::game_status_event_handler::GameStatusEventHandler;
use crate::game::application::round_status_event_handler::RoundStatusEventHandler;
use crate::robot::application::robot_application_service::RobotApplicationService;
use crate::robot::application::robot_spawned_event_handler::RobotSpawnedEventHandler;
use crate::robot::application::robots_revealed_event_handler::RobotsRevealedEventHandler;
use crate::player::application::player_application_service::PlayerApplicationService;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

use super::robot::robot_resource_mined_event;
use super::trading::bank_account_initialized_event;

pub struct EventDispatcher {
    game_status_event_handler: GameStatusEventHandler,
    round_status_event_handler: RoundStatusEventHandler,
    robot_spawned_event_handler: RobotSpawnedEventHandler,
    robots_revealed_event_handler: RobotsRevealedEventHandler,
}

impl EventDispatcher {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        game_application_service: Arc<GameApplicationService>,
        player_application_service: Arc<PlayerApplicationService>,
        robot_application_service: Arc<RobotApplicationService>,
    ) -> Self {
        Self {
            game_status_event_handler: GameStatusEventHandler::new(
                game_service_rest_adapter.clone(),
                game_application_service.clone(),
                player_application_service.clone(),
            ),
            round_status_event_handler: RoundStatusEventHandler::new(
                game_service_rest_adapter.clone(),
                game_application_service.clone(),
            ),
            robot_spawned_event_handler: RobotSpawnedEventHandler::new(
                robot_application_service.clone(), // this needs game_logic now... we will see
            ),
            robots_revealed_event_handler: RobotsRevealedEventHandler::new(
                robot_application_service.clone(), // this needs game_logic now... we will see
            )
            //TODO: add Event Handler for remaining Events
        }
    }
    pub async fn dispatch(&mut self, event: GameEvent) {
        match event.event_body {
            GameEventBodyType::GameStatus(game_status_event) => {
                self.game_status_event_handler
                    .handle(game_status_event);
            }
            GameEventBodyType::RoundStatus(round_status_event) => {
                self.round_status_event_handler
                    .handle(round_status_event);
            }
            GameEventBodyType::RobotSpawned(robot_spawned_event) => {
                self.robot_spawned_event_handler.handle(robot_spawned_event);
            }
            GameEventBodyType::RobotsRevealed(robots_revealed_event) => {
                self.robots_revealed_event_handler.handle(robots_revealed_event);
            }
            GameEventBodyType::PlanetResourceMined(planet_resource_mined_event) => {
                self.resource_mined_event_handler.handle(planet_resource_mined_event);
            }
            GameEventBodyType::PlanetDiscovered(planet_discovered_event) => {
                self.planet_discovered_handler.handle(planet_discovered_event);
            }
            GameEventBodyType::RobotResourceMined(robot_resource_mined_event) => {
                self.robot_resource_mined_handler.handle(robot_resource_mined_event);
            }
            GameEventBodyType::RobotResourceRemoved(robot_resource_removed_event) => {
                self.robot_resource_removed_handler.handle(robot_resource_removed_event);
            }
            GameEventBodyType::BankAccountInitialized(bank_account_initialized_event) => {
                self.bank_account_init_handler.handle(bank_account_initialized_event);
            }

            //TODO: Call Event Handler for Remaining Event Type
            // handlers for other events
            _ => {}
        }
    }
}
