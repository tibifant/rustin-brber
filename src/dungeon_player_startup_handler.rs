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
                .map_err(|e| error!("Failed to create RabbitMQConnectionHandler {e}\n Please make sure that RabbitMQ is running and that the credentials are correct."))
                .unwrap(),
        }
    }

    pub async fn start(&mut self) {
        self.register_player().await;
        if CONFIG.dev_mode {
            self.prepare_dev_mode().await;
        }
        let event_dispatcher = self.setup_event_dispatcher();
        self.listen_for_and_handle_events(event_dispatcher).await;
    }

    async fn start_game_once_our_player_joined(game_service_rest_adapter: &Arc<dyn GameServiceRestAdapterTrait>, player_name: String, game_id: &str) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let game = game_service_rest_adapter
                .get_all_games()
                .await
                .unwrap();
            let game = game.iter().find(|game| game.game_id == game_id);
            if let Some(game) = game {
                if game.participating_players.contains(&player_name) {
                    let start_game_result = game_service_rest_adapter.start_game(game_id).await;
                    if let Err(e) = start_game_result {
                        error!("Failed to start game: {}", e);
                    }
                    break;
                }
            }
        }
    }

    async fn prepare_dev_mode(&self) {
        self.game_service_rest_adapter
            .end_all_existing_games()
            .await
            .unwrap();
        self.rabbitmq_connection_handler.purge_queue(&self.player.player_queue).await; // To make sure that no old events are in the queue.
        let game_info = self.game_service_rest_adapter
            .create_game(1, 250)
            .await
            .map_err(|e| error!("Failed to create game {e}"))
            .unwrap();
        self.game_service_rest_adapter.patch_round_duration(&game_info.game_id, 10000).await.unwrap();

        let game_service_rest_adapter = self.game_service_rest_adapter.clone();
        let player_name = self.player.name.clone();
        tokio::task::spawn(async move {
            Self::start_game_once_our_player_joined(&game_service_rest_adapter, player_name, &game_info.game_id).await;
        });
    }

    async fn listen_for_and_handle_events(&self, event_dispatcher: EventDispatcher) {
        self.rabbitmq_connection_handler
            .listen_for_and_handle_events(&self.player, event_dispatcher)
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
