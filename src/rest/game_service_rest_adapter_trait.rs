use std::error::Error;
use std::fmt::Debug;

use async_trait::async_trait;

use crate::domainprimitives::command::command::Command;
use crate::player::player::Player;
use crate::rest::response::command_info_response::CommandInfoResponse;
use crate::rest::response::created_game_info_response_body::CreatedGameInfoResponseBody;
use crate::rest::response::game_info_response_body::GameInfoResponseBody;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GameServiceRestAdapterTrait: Send + Sync + Debug {
    fn get_player_id(&self) -> Option<String>;
    async fn get_all_games(&self) -> Result<Vec<GameInfoResponseBody>, Box<dyn Error>>;
    async fn create_game(
        &self,
        max_players: u16,
        rounds: u16,
    ) -> Result<CreatedGameInfoResponseBody, Box<dyn Error>>;
    async fn join_game(&self, game_id: &str) -> Result<bool, Box<dyn Error>>;
    async fn send_command(&self, command: Command) -> Result<CommandInfoResponse, Box<dyn Error>>;
    async fn register_player(&self) -> Result<Player, Box<dyn Error>>;
    async fn patch_round_duration(&self, game_id: &str, round_duration_in_millis: u64) -> Result<(), Box<dyn Error>>;
    async fn fetch_player(&self) -> Result<Player, Box<dyn Error>>;
    async fn start_game(&self, game_id: &str) -> Result<(), Box<dyn Error>>;
    async fn end_all_existing_games(&self) -> Result<(), Box<dyn Error>>;
}
