use std::sync::Arc;

use tracing::{error, info, warn};

use crate::config::CONFIG;
use crate::game::domain::game::Game;
use crate::repository::AsyncRepository;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct GameApplicationService {
    game_repository: Box<dyn AsyncRepository<Game> + Send + Sync>,
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
}

impl GameApplicationService {
    pub fn new(
        game_repository: Box<dyn AsyncRepository<Game> + Send + Sync>,
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    ) -> Self {
        Self {
            game_repository,
            game_service_rest_adapter,
        }
    }

    pub async fn start_game(&self, game_id: &str) {
        let game = self.game_repository.get(game_id).await.unwrap();
        match game {
            Some(mut game) => {
                if game.is_started() {
                    return;
                }
                game.start_game();
                self.game_repository.save(game).await.unwrap();
            }
            None => {
                error!("Game with id {} not found", game_id)
            }
        }
    }

    pub async fn end_game(&self, game_id: &str) {
        let game = self.game_repository.get(game_id).await.unwrap();
        match game {
            Some(mut game) => {
                if game.is_ended() {
                    return;
                }
                game.end_game();
                self.game_repository.save(game).await.unwrap();
            }
            None => {
                error!("Game with id {} not found", game_id)
            }
        }
    }

    pub async fn round_started(&self, game_id: &str) {
        let game = self.game_repository.get(game_id).await.unwrap();
        match game {
            Some(mut game) => {
                game.start_round();
                self.game_repository.save(game).await.unwrap();
            }
            None => {
                error!("Game with id {} not found", game_id)
            }
        }
    }

    pub async fn fetch_and_save_remote_game(&self) -> Option<Game> {
        let games = self.game_service_rest_adapter.get_all_games().await;
        if let Err(e) = games {
            error!("Failed to fetch remote game: {}", e);
            return None;
        }
        let games = games.unwrap();

        if games.len() > 1 {
            panic!("More than one game found");
        }
        if games.is_empty() {
            info!("No games found remotely");
            return None;
        }
        let game_info = games.first().unwrap();
        let mut game = Game::from(game_info);
        game.check_if_our_player_has_joined(&game_info.participating_players, &CONFIG.player_name);
        self.game_repository.save(game.clone()).await.unwrap();
        Some(game)
    }

    pub async fn query_active_game(&self) -> Option<Game> {
        let games = self.game_repository.get_all().await;
        if let Err(e) = games {
            error!("Failed to query active game: {}", e);
            return None;
        }
        let games = games
            .unwrap()
            .iter()
            .filter(|game| !game.is_ended())
            .cloned()
            .collect::<Vec<Game>>();
        match games.len() {
            0 => {
                warn!("No active game found");
                None
            }
            1 => Some(games.first().unwrap().clone()),
            _ => {
                panic!("More than one active game found");
            }
        }
    }

    pub async fn query_and_if_needed_fetch_remote_game(&self) -> Option<Game> {
        let game = self.query_active_game().await;
        if game.is_some() {
            return game;
        }
        self.fetch_and_save_remote_game().await
    }
}
