use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEventHeader {
    pub event_id: Option<String>,
    pub version: Option<String>,
    pub player_id: Option<String>,
    pub timestamp: Option<String>,
    pub transaction_id: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    #[serde(rename = "kafka-topic")]
    pub kafka_topic: Option<String>,
}