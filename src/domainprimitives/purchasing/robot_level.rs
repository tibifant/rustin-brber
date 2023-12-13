use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RobotLevel {
    LEVEL0,
    LEVEL1,
    LEVEL2,
    LEVEL3,
    LEVEL4,
    LEVEL5,
}

impl<'de> Deserialize<'de> for RobotLevel {
    fn deserialize<D>(deserializer: D) -> Result<RobotLevel, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "0" => Ok(RobotLevel::LEVEL0),
            "1" => Ok(RobotLevel::LEVEL1),
            "2" => Ok(RobotLevel::LEVEL2),
            "3" => Ok(RobotLevel::LEVEL3),
            "4" => Ok(RobotLevel::LEVEL4),
            "5" => Ok(RobotLevel::LEVEL5),
            s => Err(serde::de::Error::custom(format!(
                "Parse Error Invalid robot level: {} can only be between 0-5",
                s
            ))),
        }
    }
}

impl Serialize for RobotLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RobotLevel::LEVEL0 => serializer.serialize_str("0"),
            RobotLevel::LEVEL1 => serializer.serialize_str("1"),
            RobotLevel::LEVEL2 => serializer.serialize_str("2"),
            RobotLevel::LEVEL3 => serializer.serialize_str("3"),
            RobotLevel::LEVEL4 => serializer.serialize_str("4"),
            RobotLevel::LEVEL5 => serializer.serialize_str("5"),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum RobotLevelError {
    #[error("Cannot upgrade robot level any further")]
    NoHigherLevelThanMaximumLevel,
}
impl RobotLevel {
    pub fn get_minimum_level() -> RobotLevel {
        RobotLevel::LEVEL0
    }

    pub fn get_maximum_level() -> RobotLevel {
        RobotLevel::LEVEL5
    }

    pub fn is_maximum_level(&self) -> bool {
        match self {
            RobotLevel::LEVEL5 => true,
            _ => false,
        }
    }

    pub fn is_minimum_level(&self) -> bool {
        match self {
            RobotLevel::LEVEL0 => true,
            _ => false,
        }
    }
    pub fn get_next_level(&self) -> Result<RobotLevel, RobotLevelError> {
        match self {
            RobotLevel::LEVEL0 => Ok(RobotLevel::LEVEL1),
            RobotLevel::LEVEL1 => Ok(RobotLevel::LEVEL2),
            RobotLevel::LEVEL2 => Ok(RobotLevel::LEVEL3),
            RobotLevel::LEVEL3 => Ok(RobotLevel::LEVEL4),
            RobotLevel::LEVEL4 => Ok(RobotLevel::LEVEL5),
            RobotLevel::LEVEL5 => Err(RobotLevelError::NoHigherLevelThanMaximumLevel),
        }
    }
    pub fn get_max_health_value_for_level(&self) -> u16 {
        match self {
            RobotLevel::LEVEL0 => 10,
            RobotLevel::LEVEL1 => 25,
            RobotLevel::LEVEL2 => 50,
            RobotLevel::LEVEL3 => 100,
            RobotLevel::LEVEL4 => 200,
            RobotLevel::LEVEL5 => 500,
        }
    }

    pub fn get_max_energy_value_for_level(&self) -> u16 {
        match self {
            RobotLevel::LEVEL0 => 20,
            RobotLevel::LEVEL1 => 30,
            RobotLevel::LEVEL2 => 40,
            RobotLevel::LEVEL3 => 60,
            RobotLevel::LEVEL4 => 100,
            RobotLevel::LEVEL5 => 200,
        }
    }

    pub fn get_energy_regen_value_for_level(&self) -> u16 {
        match self {
            RobotLevel::LEVEL0 => 4,
            RobotLevel::LEVEL1 => 6,
            RobotLevel::LEVEL2 => 8,
            RobotLevel::LEVEL3 => 10,
            RobotLevel::LEVEL4 => 15,
            RobotLevel::LEVEL5 => 20,
        }
    }

    pub fn get_attack_damage_value_for_level(&self) -> u16 {
        match self {
            RobotLevel::LEVEL0 => 1,
            RobotLevel::LEVEL1 => 2,
            RobotLevel::LEVEL2 => 5,
            RobotLevel::LEVEL3 => 10,
            RobotLevel::LEVEL4 => 20,
            RobotLevel::LEVEL5 => 50,
        }
    }

    pub fn get_mining_speed_value_for_level(&self) -> u16 {
        match self {
            RobotLevel::LEVEL0 => 2,
            RobotLevel::LEVEL1 => 5,
            RobotLevel::LEVEL2 => 10,
            RobotLevel::LEVEL3 => 15,
            RobotLevel::LEVEL4 => 20,
            RobotLevel::LEVEL5 => 40,
        }
    }
    pub fn get_mineable_resource_for_level(&self) -> MineableResourceType {
        match self {
            RobotLevel::LEVEL0 => MineableResourceType::COAL,
            RobotLevel::LEVEL1 => MineableResourceType::IRON,
            RobotLevel::LEVEL2 => MineableResourceType::GEM,
            RobotLevel::LEVEL3 => MineableResourceType::GOLD,
            RobotLevel::LEVEL4 => MineableResourceType::PLATIN,
            RobotLevel::LEVEL5 => MineableResourceType::PLATIN,
        }
    }

    pub fn get_storage_value_for_level(&self) -> u16 {
        match self {
            RobotLevel::LEVEL0 => 20,
            RobotLevel::LEVEL1 => 50,
            RobotLevel::LEVEL2 => 100,
            RobotLevel::LEVEL3 => 200,
            RobotLevel::LEVEL4 => 400,
            RobotLevel::LEVEL5 => 1000,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_maximum_level_test() {
        assert_eq!(RobotLevel::LEVEL5.is_maximum_level(), true);
        assert_eq!(RobotLevel::LEVEL4.is_maximum_level(), false);
    }

    #[test]
    fn test_max_health_values() {
        assert_eq!(RobotLevel::LEVEL0.get_max_health_value_for_level(), 10);
        assert_eq!(RobotLevel::LEVEL1.get_max_health_value_for_level(), 25);
        assert_eq!(RobotLevel::LEVEL2.get_max_health_value_for_level(), 50);
        assert_eq!(RobotLevel::LEVEL3.get_max_health_value_for_level(), 100);
        assert_eq!(RobotLevel::LEVEL4.get_max_health_value_for_level(), 200);
        assert_eq!(RobotLevel::LEVEL5.get_max_health_value_for_level(), 500);
    }

    #[test]
    fn test_max_energy_values() {
        assert_eq!(RobotLevel::LEVEL0.get_max_energy_value_for_level(), 20);
        assert_eq!(RobotLevel::LEVEL1.get_max_energy_value_for_level(), 30);
        assert_eq!(RobotLevel::LEVEL2.get_max_energy_value_for_level(), 40);
        assert_eq!(RobotLevel::LEVEL3.get_max_energy_value_for_level(), 60);
        assert_eq!(RobotLevel::LEVEL4.get_max_energy_value_for_level(), 100);
        assert_eq!(RobotLevel::LEVEL5.get_max_energy_value_for_level(), 200);
    }

    #[test]
    fn test_energy_regen_values() {
        assert_eq!(RobotLevel::LEVEL0.get_energy_regen_value_for_level(), 4);
        assert_eq!(RobotLevel::LEVEL1.get_energy_regen_value_for_level(), 6);
        assert_eq!(RobotLevel::LEVEL2.get_energy_regen_value_for_level(), 8);
        assert_eq!(RobotLevel::LEVEL3.get_energy_regen_value_for_level(), 10);
        assert_eq!(RobotLevel::LEVEL4.get_energy_regen_value_for_level(), 15);
        assert_eq!(RobotLevel::LEVEL5.get_energy_regen_value_for_level(), 20);
    }

    #[test]
    fn test_attack_damage_values() {
        assert_eq!(RobotLevel::LEVEL0.get_attack_damage_value_for_level(), 1);
        assert_eq!(RobotLevel::LEVEL1.get_attack_damage_value_for_level(), 2);
        assert_eq!(RobotLevel::LEVEL2.get_attack_damage_value_for_level(), 5);
        assert_eq!(RobotLevel::LEVEL3.get_attack_damage_value_for_level(), 10);
        assert_eq!(RobotLevel::LEVEL4.get_attack_damage_value_for_level(), 20);
        assert_eq!(RobotLevel::LEVEL5.get_attack_damage_value_for_level(), 50);
    }

    #[test]
    fn test_mining_speed_values() {
        assert_eq!(RobotLevel::LEVEL0.get_mining_speed_value_for_level(), 2);
        assert_eq!(RobotLevel::LEVEL1.get_mining_speed_value_for_level(), 5);
        assert_eq!(RobotLevel::LEVEL2.get_mining_speed_value_for_level(), 10);
        assert_eq!(RobotLevel::LEVEL3.get_mining_speed_value_for_level(), 15);
        assert_eq!(RobotLevel::LEVEL4.get_mining_speed_value_for_level(), 20);
        assert_eq!(RobotLevel::LEVEL5.get_mining_speed_value_for_level(), 40);
    }

    #[test]
    fn test_storage_values() {
        assert_eq!(RobotLevel::LEVEL0.get_storage_value_for_level(), 20);
        assert_eq!(RobotLevel::LEVEL1.get_storage_value_for_level(), 50);
        assert_eq!(RobotLevel::LEVEL2.get_storage_value_for_level(), 100);
        assert_eq!(RobotLevel::LEVEL3.get_storage_value_for_level(), 200);
        assert_eq!(RobotLevel::LEVEL4.get_storage_value_for_level(), 400);
        assert_eq!(RobotLevel::LEVEL5.get_storage_value_for_level(), 1000);
    }

    #[test]
    fn test_deserialize() {
        let level0: RobotLevel = serde_json::from_str(r#""0""#).unwrap();
        assert_eq!(level0, RobotLevel::LEVEL0);
        let level1: RobotLevel = serde_json::from_str(r#""1""#).unwrap();
        assert_eq!(level1, RobotLevel::LEVEL1);
        let level2: RobotLevel = serde_json::from_str(r#""2""#).unwrap();
        assert_eq!(level2, RobotLevel::LEVEL2);
        let level3: RobotLevel = serde_json::from_str(r#""3""#).unwrap();
        assert_eq!(level3, RobotLevel::LEVEL3);
        let level4: RobotLevel = serde_json::from_str(r#""4""#).unwrap();
        assert_eq!(level4, RobotLevel::LEVEL4);
        let level5: RobotLevel = serde_json::from_str(r#""5""#).unwrap();
        assert_eq!(level5, RobotLevel::LEVEL5);
    }
}
