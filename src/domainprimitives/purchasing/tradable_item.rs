use serde::{Deserialize, Serialize};
use crate::domainprimitives::purchasing::tradable_item_type::TradableItemType;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradableItem {
    pub name: String,
    pub price: u16,
    #[serde(rename = "type")]
    pub tradable_type: TradableItemType,
}