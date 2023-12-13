use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct GameErrorEvent {
    pub code: Option<String>,
    pub description: Option<String>,
    pub details: Option<String>,
    pub robot_id: Option<String>,
    pub player_id: Option<String>,
    pub transaction_id: Option<String>,
}
