use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RobotRestorationType {
    HealthRestore,
    EnergyRestore,
}

impl RobotRestorationType {
    fn to_string(&self) -> String {
        match self {
            RobotRestorationType::HealthRestore => "HEALTH_RESTORE".to_string(),
            RobotRestorationType::EnergyRestore => "ENERGY_RESTORE".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(
            RobotRestorationType::HealthRestore.to_string(),
            "HEALTH_RESTORE".to_string()
        );
        assert_eq!(
            RobotRestorationType::EnergyRestore.to_string(),
            "ENERGY_RESTORE".to_string()
        );
    }
}
