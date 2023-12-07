use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TradableItemType {
    ITEM,
    UPGRADE,
    RESTORATION,
    RESOURCE,
}