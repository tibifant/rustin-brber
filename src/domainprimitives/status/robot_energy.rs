use crate::domainprimitives::errors::DomainPrimitiveError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RobotEnergy {
    pub energy_amount: u16,
}

impl RobotEnergy {
    pub fn from_amount(energy_amount: u16) -> RobotEnergy {
        RobotEnergy { energy_amount }
    }

    pub fn zero() -> RobotEnergy {
        RobotEnergy { energy_amount: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.energy_amount == 0
    }

    pub fn is_greater_than(&self, other: &RobotEnergy) -> bool {
        self.energy_amount > other.energy_amount
    }

    pub fn is_greater_equal_than(&self, other: &RobotEnergy) -> bool {
        self.energy_amount >= other.energy_amount
    }

    pub fn is_less_than(&self, other: &RobotEnergy) -> bool {
        self.energy_amount < other.energy_amount
    }

    pub fn increase_by(self, other: &RobotEnergy) -> RobotEnergy {
        RobotEnergy {
            energy_amount: self.energy_amount + other.energy_amount,
        }
    }

    pub fn decrease_by(self, other: &RobotEnergy) -> Result<RobotEnergy, DomainPrimitiveError> {
        if self.is_less_than(other) {
            return Err(DomainPrimitiveError::NegativeEnergy(
                self.energy_amount,
                other.energy_amount,
            ));
        }
        Ok(RobotEnergy {
            energy_amount: self.energy_amount - other.energy_amount,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_greater_than() {
        assert!(RobotEnergy::from_amount(10).is_greater_than(&RobotEnergy::from_amount(5)));
        assert!(!RobotEnergy::from_amount(5).is_greater_than(&RobotEnergy::from_amount(10)));
        assert!(!RobotEnergy::from_amount(5).is_greater_than(&RobotEnergy::from_amount(5)));
    }

    #[test]
    fn test_is_greater_equal_than() {
        assert!(RobotEnergy::from_amount(10).is_greater_equal_than(&RobotEnergy::from_amount(5)));
        assert!(!RobotEnergy::from_amount(5).is_greater_equal_than(&RobotEnergy::from_amount(10)));
        assert!(RobotEnergy::from_amount(5).is_greater_equal_than(&RobotEnergy::from_amount(5)));
    }

    #[test]
    fn test_is_less_than() {
        assert!(!RobotEnergy::from_amount(10).is_less_than(&RobotEnergy::from_amount(5)));
        assert!(RobotEnergy::from_amount(5).is_less_than(&RobotEnergy::from_amount(10)));
        assert!(!RobotEnergy::from_amount(5).is_less_than(&RobotEnergy::from_amount(5)));
    }

    #[test]
    fn test_increase_by() {
        assert_eq!(
            RobotEnergy::from_amount(10).increase_by(&RobotEnergy::from_amount(5)),
            RobotEnergy::from_amount(15)
        );
        assert_eq!(
            RobotEnergy::from_amount(5).increase_by(&RobotEnergy::from_amount(10)),
            RobotEnergy::from_amount(15)
        );
        assert_eq!(
            RobotEnergy::from_amount(5).increase_by(&RobotEnergy::from_amount(5)),
            RobotEnergy::from_amount(10)
        );
    }

    #[test]
    fn test_decrease_by() {
        assert_eq!(
            RobotEnergy::from_amount(10)
                .decrease_by(&RobotEnergy::from_amount(5))
                .unwrap(),
            RobotEnergy::from_amount(5)
        );
        assert_eq!(
            RobotEnergy::from_amount(5)
                .decrease_by(&RobotEnergy::from_amount(10))
                .unwrap_err(),
            DomainPrimitiveError::NegativeEnergy(5, 10)
        );
        assert_eq!(
            RobotEnergy::from_amount(5)
                .decrease_by(&RobotEnergy::from_amount(5))
                .unwrap(),
            RobotEnergy::from_amount(0)
        );
    }
}
