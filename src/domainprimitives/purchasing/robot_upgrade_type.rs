use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RobotUpgradeType {
    Storage,
    Health,
    Damage,
    MiningSpeed,
    Mining,
    MaxEnergy,
    EnergyRegen,
}

impl RobotUpgradeType {
    pub fn get_all_types() -> Vec<RobotUpgradeType> {
        vec![
            RobotUpgradeType::Storage,
            RobotUpgradeType::Health,
            RobotUpgradeType::Damage,
            RobotUpgradeType::MiningSpeed,
            RobotUpgradeType::Mining,
            RobotUpgradeType::MaxEnergy,
            RobotUpgradeType::EnergyRegen,
        ]
    }

    pub fn to_string(&self) -> String {
        match self {
            RobotUpgradeType::Storage => "STORAGE".to_string(),
            RobotUpgradeType::Health => "HEALTH".to_string(),
            RobotUpgradeType::Damage => "DAMAGE".to_string(),
            RobotUpgradeType::MiningSpeed => "MINING_SPEED".to_string(),
            RobotUpgradeType::Mining => "MINING".to_string(),
            RobotUpgradeType::MaxEnergy => "ENERGY".to_string(),
            RobotUpgradeType::EnergyRegen => "ENERGY_REGEN".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(RobotUpgradeType::Storage.to_string(), "STORAGE".to_string());
        assert_eq!(RobotUpgradeType::Health.to_string(), "HEALTH".to_string());
        assert_eq!(RobotUpgradeType::Damage.to_string(), "DAMAGE".to_string());
        assert_eq!(
            RobotUpgradeType::MiningSpeed.to_string(),
            "MINING_SPEED".to_string()
        );
        assert_eq!(RobotUpgradeType::Mining.to_string(), "MINING".to_string());
        assert_eq!(
            RobotUpgradeType::MaxEnergy.to_string(),
            "ENERGY".to_string()
        );
        assert_eq!(
            RobotUpgradeType::EnergyRegen.to_string(),
            "ENERGY_REGEN".to_string()
        );
    }
}
