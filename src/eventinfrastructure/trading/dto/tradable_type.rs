use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TradableType {
    ITEM,
    UPGRADE,
    RESTORATION,
    RESOURCE,
}

impl TradableType {
    pub fn to_string(&self) -> String {
        match self {
            TradableType::ITEM => "ITEM".to_string(),
            TradableType::UPGRADE => "UPGRADE".to_string(),
            TradableType::RESTORATION => "RESTORATION".to_string(),
            TradableType::RESOURCE => "RESOURCE".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(TradableType::ITEM.to_string(), "ITEM".to_string());
        assert_eq!(TradableType::UPGRADE.to_string(), "UPGRADE".to_string());
        assert_eq!(
            TradableType::RESTORATION.to_string(),
            "RESTORATION".to_string()
        );
        assert_eq!(TradableType::RESOURCE.to_string(), "RESOURCE".to_string());
    }
}
