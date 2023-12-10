use serde::Deserialize;

use crate::eventinfrastructure::trading::dto::tradable_item_dto::TradableItemDto;

#[derive(Debug)]
pub struct TradablePricesEvent {
    pub items: Vec<TradableItemDto>,
}

impl<'de> Deserialize<'de> for TradablePricesEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let items = Vec::<TradableItemDto>::deserialize(deserializer)?;
        Ok(TradablePricesEvent { items })
    }
}


