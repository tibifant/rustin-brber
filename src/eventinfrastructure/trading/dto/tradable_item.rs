use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::trading::dto::tradable_type::TradableType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradableItem {
    pub name: String,
    pub price: u16,
    #[serde(rename = "type")]
    pub tradable_type: TradableType,
}
