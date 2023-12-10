use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TradableItemTypeDto {
    ITEM,
    UPGRADE,
    RESTORATION,
    RESOURCE,
}