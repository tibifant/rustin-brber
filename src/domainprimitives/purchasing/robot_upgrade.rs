use crate::domainprimitives::purchasing::robot_level::{RobotLevel, RobotLevelError};
use crate::domainprimitives::purchasing::robot_upgrade_type::RobotUpgradeType;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RobotUpgrade {
    pub upgrade_type: RobotUpgradeType,
    pub level: RobotLevel,
}

impl RobotUpgrade {
    pub fn base_for_type(upgrade_type: RobotUpgradeType) -> RobotUpgrade {
        RobotUpgrade {
            upgrade_type,
            level: RobotLevel::get_minimum_level(),
        }
    }

    pub fn for_type_and_level(upgrade_type: RobotUpgradeType, level: RobotLevel) -> RobotUpgrade {
        RobotUpgrade {
            upgrade_type,
            level,
        }
    }

    pub fn get_all_base_upgrades() -> Vec<RobotUpgrade> {
        let mut upgrades = Vec::new();
        for upgrade_type in RobotUpgradeType::get_all_types() {
            upgrades.push(RobotUpgrade::base_for_type(upgrade_type));
        }
        upgrades
    }

    pub fn get_next_level(&self) -> Result<RobotLevel, RobotLevelError> {
        self.level.get_next_level()
    }

    pub fn to_string_for_command(&self) -> String {
        let level = match self.level {
            RobotLevel::LEVEL0 => "0",
            RobotLevel::LEVEL1 => "1",
            RobotLevel::LEVEL2 => "2",
            RobotLevel::LEVEL3 => "3",
            RobotLevel::LEVEL4 => "4",
            RobotLevel::LEVEL5 => "5",
        };
        let upgrade_type = match self.upgrade_type {
            RobotUpgradeType::Health => "HEALTH",
            RobotUpgradeType::MaxEnergy => "ENERGY",
            RobotUpgradeType::EnergyRegen => "ENERGY_REGEN",
            RobotUpgradeType::Damage => "DAMAGE",
            RobotUpgradeType::MiningSpeed => "MINING_SPEED",
            RobotUpgradeType::Mining => "MINING",
            RobotUpgradeType::Storage => "STORAGE",
        };
        format!("{}_{}", upgrade_type, level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_level() {
        let upgrade = RobotUpgrade::base_for_type(RobotUpgradeType::Health);
        assert_eq!(upgrade.get_next_level().unwrap(), RobotLevel::LEVEL1);
    }

    #[test]
    fn test_get_next_level_2() {
        let upgrade =
            RobotUpgrade::for_type_and_level(RobotUpgradeType::Health, RobotLevel::LEVEL2);
        assert_eq!(upgrade.get_next_level().unwrap(), RobotLevel::LEVEL3);
    }

    #[test]
    fn get_for_type_and_level() {
        let upgrade =
            RobotUpgrade::for_type_and_level(RobotUpgradeType::Health, RobotLevel::LEVEL2);
        assert_eq!(upgrade.upgrade_type, RobotUpgradeType::Health);
        assert_eq!(upgrade.level, RobotLevel::LEVEL2);
    }

    #[test]
    fn test_get_next_level_max() {
        let upgrade =
            RobotUpgrade::for_type_and_level(RobotUpgradeType::Health, RobotLevel::LEVEL5);
        assert_eq!(
            upgrade.get_next_level().unwrap_err(),
            RobotLevelError::NoHigherLevelThanMaximumLevel
        );
    }

    #[test]
    fn test_to_string_for_command() {
        let upgrade =
            RobotUpgrade::for_type_and_level(RobotUpgradeType::Health, RobotLevel::LEVEL5);
        assert_eq!(upgrade.to_string_for_command(), "HEALTH_5");
    }
}
