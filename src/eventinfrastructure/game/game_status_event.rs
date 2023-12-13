use serde::{Deserialize, Serialize};

use crate::game::domain::game_status::GameStatus;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStatusEvent {
    pub game_id: String,
    pub gameworld_id: Option<String>,
    pub status: GameStatus,
}
