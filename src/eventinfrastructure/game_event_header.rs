use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEventHeader {
    pub event_id: String,
    pub version: String,
    pub player_id: String,
    pub timestamp: String,
    pub transaction_id: String,
    pub event_type: String,
    #[serde(rename = "kafka-topic")]
    pub kafka_topic: String,
}