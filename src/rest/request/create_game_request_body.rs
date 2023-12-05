use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameRequestBody {
    pub max_players: u16,
    pub max_rounds: u16,
}
