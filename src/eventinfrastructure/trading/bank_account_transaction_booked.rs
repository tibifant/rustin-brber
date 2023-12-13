use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountTransactionBookedEvent {
    pub player_id: String,
    pub balance: f32,
    pub transaction_amount: f32,
}
