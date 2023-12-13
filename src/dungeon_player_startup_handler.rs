use std::sync::Arc;

use tracing::{error, info};

use crate::config::CONFIG;
use crate::eventinfrastructure::event_dispatcher::EventDispatcher;
use crate::eventinfrastructure::rabbitmq::rabbitmq_connection_handler::RabbitMQConnectionHandler;
use crate::player::player::Player;
use crate::rest::game_service_rest_adapter_impl::*;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct DungeonPlayerStartupHandler {
    player: Player,
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    rabbitmq_connection_handler: RabbitMQConnectionHandler,
}

impl DungeonPlayerStartupHandler {
    pub async fn new() -> Self {
        Self {
            player: Player::new(),
            game_service_rest_adapter: Arc::new(GameServiceRestAdapterImpl::new()),
            rabbitmq_connection_handler: RabbitMQConnectionHandler::new()
                .await
                .map_err(|e| error!("Failed to create RabbitMQConnectionHandler {e}"))
                .unwrap(),
        }
    }

    pub async fn start(&mut self) {
        self.register_player().await;
        let is_in_dev_mode = CONFIG.dev_mode;
        if is_in_dev_mode {
            self.game_service_rest_adapter
                .end_all_existing_games()
                .await
                .unwrap();
        }
        let event_dispatcher = self.setup_event_dispatcher();
        self.listen_for_events(event_dispatcher).await;
        if is_in_dev_mode {
            self.game_service_rest_adapter
                .create_game(10, 250)
                .await
                .map_err(|e| error!("Failed to create game {e}"))
                .unwrap();
        }
    }

    async fn listen_for_events(&self, event_dispatcher: EventDispatcher) {
        self.rabbitmq_connection_handler
            .listen_for_events(&self.player, event_dispatcher)
            .await;
    }

    fn setup_event_dispatcher(&mut self) -> EventDispatcher {
        EventDispatcher::new(self.game_service_rest_adapter.clone())
    }
    async fn register_player(&mut self) {
        let player_response = self
            .game_service_rest_adapter
            .register_player()
            .await
            .map_err(|e| error!("Failed to register player {e}"))
            .unwrap();
        let new_game_service_rest_adapter = GameServiceRestAdapterImpl::new()
            .with_player_id(player_response.player_id.clone().unwrap());
        self.game_service_rest_adapter = Arc::new(new_game_service_rest_adapter);
        self.player = player_response;
        info!("Registered player: {:?}", &self.player);
    }
}
