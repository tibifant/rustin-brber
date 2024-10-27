use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::{error, info};

use crate::game_logic;
use crate::game_logic::GameLogic;
use crate::player::domain::player::Player;
use crate::repository::AsyncRepository;
use crate::repository::InMemoryRepository;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct PlayerApplicationService {
    player_repository: Box<dyn AsyncRepository<Player> + Send + Sync>,
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    game_logic: Arc<Mutex<GameLogic>>,
}

impl PlayerApplicationService {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        game_logic: Arc<Mutex<GameLogic>>,
    ) -> Self {
        Self {
            player_repository: Box::new(InMemoryRepository::new()),
            game_service_rest_adapter,
            game_logic
        }
    }
    pub async fn query_and_if_needed_create_player(&self) -> Player {
        let players = self.player_repository.get_all().await.unwrap();
        if players.len() > 1 {
            panic!("More than one player found");
        }
        if players.is_empty() {
            info!("No player found, creating new player");
            Player::new()
        } else {
            players.first().unwrap().clone()
        }
    }

    pub async fn register_player(&self) -> Player {
        let mut player = self.query_and_if_needed_create_player().await;
        if player.is_registered() {
            info!("Player is already registered");
            return player;
        }
        let remote_player_id = self.game_service_rest_adapter.get_player_id().await;
        if let Some(remote_player_id) = remote_player_id {
            info!("Player is already registered remotely, saving player locally");
            player.assign_player_id(remote_player_id.clone());
            self.game_logic.lock().await.game_data.player_id = remote_player_id.clone();
            self.player_repository.save(player.clone()).await.unwrap();
            return player;
        } else {
            info!("Player is not registered yet. Registering Player and saving him locally");
            let player = self
                .game_service_rest_adapter
                .register_player()
                .await
                .unwrap();
            self.player_repository.save(player.clone()).await.unwrap();
            player
        }
    }

    pub async fn join_game(&self, game_id: &str) -> bool {
        let mut player = self.register_player().await;
        if player.game_id.is_some() {
            error!("Player is already in a game, cannot join another one");
            return false;
        }
        let join_successful = self
            .game_service_rest_adapter
            .join_game(&game_id)
            .await
            .unwrap();
        if join_successful {
            player.assign_game_id(game_id.to_string());
            self.player_repository.save(player.clone()).await.unwrap();
            info!("Player joined game {}", game_id);
            return true;
        }
        false
    }

    pub async fn clear_game_id(&self) {
        let mut player = self.query_and_if_needed_create_player().await;
        player.game_id = None;
        self.player_repository.update(player.clone()).await.unwrap();
    }
}
