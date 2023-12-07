use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::trading::dto::tradable_item_type::TradableItemType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradableSoldEvent {
    pub player_id: String,
    pub robot_id: String,
    #[serde(rename = "type")]
    pub tradable_type : TradableItemType,
    pub name: String,
    pub amount: u32,
    pub price_per_unit: f32,
    pub total_price: f32,
}