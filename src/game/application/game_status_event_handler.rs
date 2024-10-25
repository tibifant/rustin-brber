use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info};

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game::game_status_event::GameStatusEvent;
use crate::game::application::game_application_service::GameApplicationService;
use crate::game::domain::game_status::GameStatus;
use crate::player::application::player_application_service::PlayerApplicationService;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct GameStatusEventHandler {
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    game_application_service: Arc<GameApplicationService>,
    player_application_service: Arc<PlayerApplicationService>,
}

impl GameStatusEventHandler {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        game_application_service: Arc<GameApplicationService>,
        player_application_service: Arc<PlayerApplicationService>,
    ) -> Self {
        Self {
            game_service_rest_adapter,
            game_application_service,
            player_application_service,
        }
    }
}

#[async_trait]
impl EventHandler<GameStatusEvent> for GameStatusEventHandler {
    async fn handle(&mut self, event: GameStatusEvent) {
        match event.status {
            GameStatus::CREATED => {
                info!("Game {} Status: Created", event.game_id);
                self.game_application_service
                    .fetch_and_save_remote_game()
                    .await;
                let joined_game_successfully = self
                    .player_application_service
                    .join_game(&event.game_id)
                    .await;
                if !joined_game_successfully {
                    error!("Error joining game: {}", event.game_id);
                }
            }
            GameStatus::STARTED => {
                info!("Game {} Status: Started", event.game_id);
                self.game_application_service
                    .start_game(&event.game_id)
                    .await;
            }
            GameStatus::ENDED => {
                info!("Game {} Status: Ended", event.game_id);
                self.game_application_service.end_game(&event.game_id).await;
                self.player_application_service.clear_game_id().await;
            }
        }
    }
}
