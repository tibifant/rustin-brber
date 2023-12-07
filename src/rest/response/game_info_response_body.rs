use serde::{Deserialize, Serialize};
use crate::rest::game_service_rest_adapter::GameStatus;

#[derive(Serialize,Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameInfoResponseBody {
    pub game_id: String,
    pub game_status: GameStatus,
    pub max_players: u16,
    pub max_rounds: u16,
    pub current_round_number: Option<u16>,
    pub round_length_in_millis: u16,
    pub participating_players: Vec<String>,
}