use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountClearedEvent {
    pub player_id: String,
    pub balance: f32,
}
