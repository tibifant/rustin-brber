use super::robot_upgrade_type::RobotUpgradeType;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum TradeItemType {
  MiningSpeed1,
  MiningSpeed2,
  MiningSpeed3,
  MiningSpeed4,
  MiningSpeed5,
  MaxEnergy1,
  MaxEnergy2,
  MaxEnergy3,
  MaxEnergy4,
  MaxEnergy5,
  EnergyRegen1,
  EnergyRegen2,
  EnergyRegen3,
  EnergyRegen4,
  EnergyRegen5,
  Storage1,
  Storage2,
  Storage3,
  Storage4,
  Storage5,
  Mining1,
  Mining2,
  Mining3,
  Mining4,
  Mining5,
  Health1,
  Health2,
  Health3,
  Health4,
  Health5,
  Damage1,
  Damage2,
  Damage3,
  Damage4,
  Damage5,
  EnergyRestore,
  HealthRestore,
  Robot,
}

impl TradeItemType {
  pub fn get_next_level_item(upgrade_type: RobotUpgradeType, current_level: u16) -> Option<TradeItemType> {
    match upgrade_type {
        RobotUpgradeType::Storage => match current_level {
            1 => Some(TradeItemType::Storage2),
            2 => Some(TradeItemType::Storage3),
            3 => Some(TradeItemType::Storage4),
            4 => Some(TradeItemType::Storage5),
            _ => None,
        },
        RobotUpgradeType::Health => match current_level {
            1 => Some(TradeItemType::Health2),
            2 => Some(TradeItemType::Health3),
            3 => Some(TradeItemType::Health4),
            4 => Some(TradeItemType::Health5),
            _ => None,
        },
        RobotUpgradeType::Damage => match current_level {
            1 => Some(TradeItemType::Damage2),
            2 => Some(TradeItemType::Damage3),
            3 => Some(TradeItemType::Damage4),
            4 => Some(TradeItemType::Damage5),
            _ => None,
        },
        RobotUpgradeType::MiningSpeed => match current_level {
            1 => Some(TradeItemType::MiningSpeed2),
            2 => Some(TradeItemType::MiningSpeed3),
            3 => Some(TradeItemType::MiningSpeed4),
            4 => Some(TradeItemType::MiningSpeed5),
            _ => None,
        },
        RobotUpgradeType::Mining => match current_level {
            1 => Some(TradeItemType::Mining2),
            2 => Some(TradeItemType::Mining3),
            3 => Some(TradeItemType::Mining4),
            4 => Some(TradeItemType::Mining5),
            _ => None,
        },
        RobotUpgradeType::MaxEnergy => match current_level {
            1 => Some(TradeItemType::MaxEnergy2),
            2 => Some(TradeItemType::MaxEnergy3),
            3 => Some(TradeItemType::MaxEnergy4),
            4 => Some(TradeItemType::MaxEnergy5),
            _ => None,
        },
        RobotUpgradeType::EnergyRegen => match current_level {
            1 => Some(TradeItemType::EnergyRegen2),
            2 => Some(TradeItemType::EnergyRegen3),
            3 => Some(TradeItemType::EnergyRegen4),
            4 => Some(TradeItemType::EnergyRegen5),
            _ => None,
        },
    }
  }
}