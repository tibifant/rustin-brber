use crate::domainprimitives::errors::DomainPrimitiveError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Money {
    amount: u64,
}

impl Money {
    pub fn from_amount(amount: u64) -> Money {
        Money { amount }
    }

    pub fn zero() -> Money {
        Money { amount: 0 }
    }

    pub fn can_buy_that_many_for(&self, price: &Money) -> u64 {
        self.amount / price.amount
    }

    pub fn is_less_than(&self, other: &Money) -> bool {
        self.amount < other.amount
    }

    pub fn is_greater_than(&self, other: &Money) -> bool {
        self.amount > other.amount
    }

    pub fn is_greater_equal_than(&self, other: &Money) -> bool {
        self.amount >= other.amount
    }

    pub fn increase_by(self, other: &Money) -> Money {
        Money {
            amount: self.amount + other.amount,
        }
    }

    pub fn decrease_by(self, other: &Money) -> Result<Money, DomainPrimitiveError> {
        if self.is_less_than(other) {
            return Err(DomainPrimitiveError::NegativeMoney(
                self.amount,
                other.amount,
            ));
        }
        Ok(Money {
            amount: self.amount - other.amount,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_greater_than() {
        assert!(Money::from_amount(10).is_greater_than(&Money::from_amount(5)));
        assert!(!Money::from_amount(5).is_greater_than(&Money::from_amount(10)));
        assert!(!Money::from_amount(5).is_greater_than(&Money::from_amount(5)));
    }

    #[test]
    fn test_is_greater_equal_than() {
        assert!(Money::from_amount(10).is_greater_equal_than(&Money::from_amount(5)));
        assert!(!Money::from_amount(5).is_greater_equal_than(&Money::from_amount(10)));
        assert!(Money::from_amount(5).is_greater_equal_than(&Money::from_amount(5)));
    }

    #[test]
    fn test_is_less_than() {
        assert!(!Money::from_amount(10).is_less_than(&Money::from_amount(5)));
        assert!(Money::from_amount(5).is_less_than(&Money::from_amount(10)));
        assert!(!Money::from_amount(5).is_less_than(&Money::from_amount(5)));
    }

    #[test]
    fn test_increase_by() {
        assert_eq!(
            Money::from_amount(10).increase_by(&Money::from_amount(5)),
            Money::from_amount(15)
        );
    }

    #[test]
    fn test_decrease_by() {
        assert_eq!(
            Money::from_amount(10)
                .decrease_by(&Money::from_amount(5))
                .unwrap(),
            Money::from_amount(5)
        );
    }

    #[test]
    fn test_decrease_by_negative_throws_error() {
        assert_eq!(
            Money::from_amount(10).decrease_by(&Money::from_amount(15)),
            Err(DomainPrimitiveError::NegativeMoney(10, 15))
        );
    }

    #[test]
    fn test_can_buy_that_many_for() {
        assert_eq!(
            Money::from_amount(11).can_buy_that_many_for(&Money::from_amount(5)),
            2
        );
        assert_eq!(
            Money::from_amount(10).can_buy_that_many_for(&Money::from_amount(5)),
            2
        );
        assert_eq!(
            Money::from_amount(9).can_buy_that_many_for(&Money::from_amount(5)),
            1
        );
        assert_eq!(
            Money::from_amount(100005).can_buy_that_many_for(&Money::from_amount(12)),
            8333
        );
    }
}
