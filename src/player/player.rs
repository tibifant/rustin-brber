use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub player_id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub player_exchange: String,
    pub player_queue: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            player_id : None,
            name: CONFIG.player_name.clone(),
            email: CONFIG.player_email.clone(),
            player_exchange: format!("player-{}", CONFIG.player_name),
            player_queue: format!("player-{}", CONFIG.player_name),
        }
    }

    pub fn initialize_id(&mut self) {
        self.player_id = Some(Uuid::new_v4());
    }
}
