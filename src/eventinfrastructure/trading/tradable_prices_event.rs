use serde::Deserialize;

use crate::eventinfrastructure::trading::dto::tradable_item::TradableItem;

#[derive(Debug)]
pub struct TradablePricesEvent {
    pub items: Vec<TradableItem>,
}

impl<'de> Deserialize<'de> for TradablePricesEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let items = Vec::<TradableItem>::deserialize(deserializer)?;
        Ok(TradablePricesEvent { items })
    }
}
