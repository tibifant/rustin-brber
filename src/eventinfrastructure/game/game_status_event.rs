use serde::{Deserialize, Serialize};

use crate::rest::game_service_rest_adapter_impl::GameStatus;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStatusEvent {
    pub game_id: String,
    pub gameworld_id: Option<String>,
    pub status: GameStatus,
}

