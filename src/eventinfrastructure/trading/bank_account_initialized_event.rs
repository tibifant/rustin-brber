
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountInitializedEvent{
    pub balance: f32,
    pub player_id: String,
}