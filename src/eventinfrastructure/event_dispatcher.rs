use std::sync::Arc;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game::handler::game_status_event_handler::GameStatusEventHandler;
use crate::eventinfrastructure::game::handler::round_status_event_handler::RoundStatusEventHandler;
use crate::eventinfrastructure::game_event::GameEvent;
use crate::eventinfrastructure::game_event_body_type::GameEventBodyType;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct EventDispatcher {
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    game_status_event_handler: Arc<GameStatusEventHandler>,
    round_status_event_handler: Arc<RoundStatusEventHandler>,
}

impl EventDispatcher {
    pub fn new(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>) -> Self {
        Self {
            game_service_rest_adapter: game_service_rest_adapter.clone(),
            game_status_event_handler: Arc::new(GameStatusEventHandler::new(
                game_service_rest_adapter.clone(),
            )),
            round_status_event_handler: Arc::new(RoundStatusEventHandler::new(
                game_service_rest_adapter.clone(),
            )),
            //TODO: add Event Handler for remaining
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
            //TODO: Call Event Handler for Remaining Event Type
            _ => {}
        }
    }
}
