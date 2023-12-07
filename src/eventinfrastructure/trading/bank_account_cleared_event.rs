use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountClearedEvent{
    pub player_id: String,
    pub balance: f32,
}