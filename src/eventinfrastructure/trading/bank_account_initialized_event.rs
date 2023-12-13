use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountInitializedEvent {
    pub balance: f32,
    pub player_id: String,
}
