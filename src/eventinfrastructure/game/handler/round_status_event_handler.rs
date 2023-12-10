use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game::dto::round_status_dto::RoundStatusDto;
use crate::eventinfrastructure::game::round_status_event::RoundStatusEvent;
use crate::rest::game_service_rest_adapter::GameServiceRestAdapterTrait;

#[derive(Debug)]
pub struct RoundStatusEventHandler {
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
}

impl RoundStatusEventHandler {
    pub fn new(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>) -> Self {
        Self {
            game_service_rest_adapter,
        }
    }
}
#[async_trait]
impl EventHandler<RoundStatusEvent> for RoundStatusEventHandler {
    async fn handle(&self, event: RoundStatusEvent) {
        match event.round_status {
            RoundStatusDto::Started => info!("Round {} Status: Running", event.round_number),
            RoundStatusDto::CommandInputEnded => info!("Round {} Status: Command Input Ended", event.round_number),
            RoundStatusDto::Ended => info!("Round {} Status: Ended", event.round_number),
        }
    }
}
