use serde::{Deserialize, Serialize};

use crate::config::CONFIG;
use crate::repository::Identifiable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub player_id: Option<String>,
    pub game_id: Option<String>,
    pub name: String,
    pub email: String,
    pub player_exchange: String,
    pub player_queue: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            player_id: None,
            game_id: None,
            name: CONFIG.player_name.clone(),
            email: CONFIG.player_email.clone(),
            player_exchange: format!("player-{}", CONFIG.player_name),
            player_queue: format!("player-{}", CONFIG.player_name),
        }
    }

    pub fn is_registered(&self) -> bool {
        self.player_id.is_some()
    }

    pub fn assign_player_id(&mut self, player_id: String) {
        self.player_id = Some(player_id);
    }

    pub fn assign_game_id(&mut self, game_id: String) {
        self.game_id = Some(game_id);
    }
}

impl Identifiable for Player {
    fn id(&self) -> String {
        self.player_id.clone().expect("Player id is not set")
    }
}
