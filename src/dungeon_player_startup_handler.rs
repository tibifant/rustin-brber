use std::sync::Arc;

use tracing::error;

use crate::config::CONFIG;
use crate::eventinfrastructure::event_dispatcher::EventDispatcher;
use crate::eventinfrastructure::rabbitmq::rabbitmq_connection_handler::RabbitMQConnectionHandler;
use crate::game::application::game_application_service::GameApplicationService;
use crate::player::application::player_application_service::{self, PlayerApplicationService};
use crate::player::domain::player::Player;
use crate::repository::InMemoryRepository;
use crate::rest::game_service_rest_adapter_impl::*;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;
use crate::robot::application::robot_application_service::{self, RobotApplicationService};

pub struct DungeonPlayerStartupHandler {
    player_application_service: Arc<PlayerApplicationService>,
    robot_application_service: Arc<RobotApplicationService>,
    game_application_service: Arc<GameApplicationService>,
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    rabbitmq_connection_handler: RabbitMQConnectionHandler,
}

impl DungeonPlayerStartupHandler {
    pub async fn new() -> Self {
        let game_service_rest_adapter = Arc::new(GameServiceRestAdapterImpl::new());
        let player_application_service = Arc::new(PlayerApplicationService::new(
            game_service_rest_adapter.clone()));
        let robot_application_service = Arc::new(RobotApplicationService::new(
            game_service_rest_adapter.clone(),
            player_application_service.clone(),
        ));
        Self {
            player_application_service: player_application_service.clone(),
            robot_application_service: robot_application_service.clone(),
            game_application_service: Arc::new(GameApplicationService::new(
                Box::new(InMemoryRepository::new()),
                game_service_rest_adapter.clone(),
                player_application_service.clone(),
                robot_application_service.clone(),
            )),
            game_service_rest_adapter: game_service_rest_adapter.clone(),
            rabbitmq_connection_handler: RabbitMQConnectionHandler::new()
                .await
                .map_err(|e| error!("Failed to create RabbitMQConnectionHandler {e}\n Please make sure that RabbitMQ is running and that the credentials are correct."))
                .unwrap(),
        }
    }

    pub async fn start(&mut self) {
        let player = self.player_application_service.register_player().await;
        self.prepare_dev_mode().await;
        let event_dispatcher = self.setup_event_dispatcher();
        self.rabbitmq_connection_handler
            .purge_queue(&player.player_queue)
            .await;

        if let Some(potential_game) = self
            .game_application_service
            .fetch_and_save_remote_game()
            .await
        {
            self.player_application_service
                .join_game(&potential_game.game_id)
                .await;
        }

        self.listen_for_and_handle_events(player, event_dispatcher)
            .await;
    }

    fn setup_event_dispatcher(&mut self) -> EventDispatcher {
        EventDispatcher::new(
            self.game_service_rest_adapter.clone(),
            self.game_application_service.clone(),
            self.player_application_service.clone(),
            self.robot_application_service.clone(),
        )
    }

    async fn listen_for_and_handle_events(
        &self,
        player: Player,
        event_dispatcher: EventDispatcher,
    ) {
        self.rabbitmq_connection_handler
            .listen_for_and_handle_events(&player, event_dispatcher)
            .await;
    }

    async fn prepare_dev_mode(&self) {
        if CONFIG.dev_mode {
            self.game_service_rest_adapter
                .end_all_existing_games()
                .await
                .map_err(|e| error!("Failed to end all existing games {e}"))
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let game_info = self
                .game_service_rest_adapter
                .create_game(1, 250)
                .await
                .map_err(|e| error!("Failed to create game {e}"))
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            self.game_service_rest_adapter
                .patch_round_duration(&game_info.game_id, 10000)
                .await
                .map_err(|e| error!("Failed to patch round duration {e}"))
                .unwrap();
            let player_name = self
                .player_application_service
                .query_and_if_needed_create_player()
                .await
                .name;
            let game_service_rest_adapter = self.game_service_rest_adapter.clone();
            tokio::task::spawn(async move {
                Self::start_game_once_our_player_joined(
                    &game_service_rest_adapter,
                    player_name,
                    &game_info.game_id,
                )
                .await;
            });
        }
    }

    async fn start_game_once_our_player_joined(
        game_service_rest_adapter: &Arc<dyn GameServiceRestAdapterTrait>,
        player_name: String,
        game_id: &str,
    ) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let game = game_service_rest_adapter.get_all_games().await.unwrap();
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
}
