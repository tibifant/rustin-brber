use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::trading::dto::tradable_item_type_dto::TradableItemTypeDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradableItemDto {
    pub name: String,
    pub price: u16,
    #[serde(rename = "type")]
    pub tradable_type: TradableItemTypeDto,
}