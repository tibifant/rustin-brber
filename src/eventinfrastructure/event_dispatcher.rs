use std::sync::Arc;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game_event::GameEvent;
use crate::eventinfrastructure::game_event_body_type::GameEventBodyType;
use crate::game::application::game_application_service::GameApplicationService;
use crate::game::application::game_status_event_handler::GameStatusEventHandler;
use crate::game::application::round_status_event_handler::RoundStatusEventHandler;
use crate::game::application::robot_spawned_event_handler::RobotSpawnedEventHandler;
use crate::player::application::player_application_service::PlayerApplicationService;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct EventDispatcher {
    game_status_event_handler: Arc<GameStatusEventHandler>,
    round_status_event_handler: Arc<RoundStatusEventHandler>,
    robot_spawned_event_handler: Arc<RobotSpawnedEventHandler>,
}

impl EventDispatcher {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        game_application_service: Arc<GameApplicationService>,
        player_application_service: Arc<PlayerApplicationService>,
    ) -> Self {
        Self {
            game_status_event_handler: Arc::new(GameStatusEventHandler::new(
                game_service_rest_adapter.clone(),
                game_application_service.clone(),
                player_application_service.clone(),
            )),
            round_status_event_handler: Arc::new(RoundStatusEventHandler::new(
                game_service_rest_adapter.clone(),
                game_application_service.clone(),
            )),
            robot_spawned_event_handler: Arc::new(RobotSpawnedEventHandler::new(
                game_service_rest_adapter.clone(),
                game_application_service.clone(),
            )),
            //TODO: add Event Handler for remaining Events
        }
    }
    pub async fn dispatch(&self, event: GameEvent) {
        match event.event_body {
            GameEventBodyType::GameStatus(game_status_event) => {
                self.game_status_event_handler
                    .handle(game_status_event)
                    .await;
            }
            GameEventBodyType::RoundStatus(round_status_event) => {
                self.round_status_event_handler
                    .handle(round_status_event)
                    .await;
            }
            GameEventBodyType::RobotSpawned(robot_spawned_event) => {
                self.robot_spawned_event_handler.handle(robot_spawned_event).await;
            }
            //TODO: Call Event Handler for Remaining Event Type
            // handlers for other events
            _ => {}
        }
    }
}
