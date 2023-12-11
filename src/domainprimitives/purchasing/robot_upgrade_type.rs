use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RobotUpgradeType {
    Storage,
    Health,
    Damage,
    MiningSpeed,
    Mining,
    MaxEnergy,
    EnergyRegen
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
            RobotUpgradeType::EnergyRegen
        ]
    }
}