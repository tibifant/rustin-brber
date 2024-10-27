use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game::dto::round_status_dto::RoundStatusDto;
use crate::eventinfrastructure::game::round_status_event::RoundStatusEvent;
use crate::game::application::game_application_service::GameApplicationService;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct RoundStatusEventHandler {
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    game_application_service: Arc<GameApplicationService>,
}

impl RoundStatusEventHandler {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        game_application_service: Arc<GameApplicationService>,
    ) -> Self {
        Self {
            game_service_rest_adapter,
            game_application_service,
        }
    }
}

#[async_trait]
impl EventHandler<RoundStatusEvent> for RoundStatusEventHandler {
    async fn handle(&mut self, event: RoundStatusEvent) {
        match event.round_status {
            RoundStatusDto::Started => {
                info!("Round {} started.", event.round_number);
                self.game_application_service
                    .round_started(&event.game_id)
                    .await;
            }
            RoundStatusDto::CommandInputEnded => {
                info!("Round {} Command Input Ended", event.round_number)
            }
            RoundStatusDto::Ended => info!("Round {} Ended", event.round_number),
        }
    }
}
