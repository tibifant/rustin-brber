use serde::{Deserialize, Serialize};

use crate::domainprimitives::errors::DomainPrimitiveError;
use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MineableResource {
    pub resource_type: MineableResourceType,
    pub max_amount: u32,
    pub current_amount: u32,
}

impl MineableResource {
    pub fn from_type_amount_and_max_amount(
        resource_type: MineableResourceType,
        current_amount: u32,
        max_amount: u32,
    ) -> Self {
        Self {
            resource_type,
            max_amount,
            current_amount,
        }
    }

    pub fn add(self, additional_resource: Self) -> Result<MineableResource, DomainPrimitiveError> {
        if additional_resource.is_empty() {
            return Ok(self);
        }
        if self.is_empty() {
            return Ok(additional_resource);
        }
        if (self.resource_type != additional_resource.resource_type) {
            return Err(DomainPrimitiveError::InvalidResourceType(
                self.resource_type,
                additional_resource.resource_type,
            ));
        }
        return Ok(Self {
            resource_type: self.resource_type,
            max_amount: self.max_amount,
            current_amount: self.current_amount + additional_resource.current_amount,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.current_amount == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let resource1 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 10, 100);
        let resource2 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 20, 100);
        let resource3 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::IRON, 20, 100);
        let resource4 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 0, 100);
        let resource5 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 0, 100);

        assert_eq!(
            resource1.add(resource2).unwrap(),
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 30, 100)
        );
        assert_eq!(
            resource1.add(resource3).unwrap_err(),
            DomainPrimitiveError::InvalidResourceType(
                MineableResourceType::COAL,
                MineableResourceType::IRON
            )
        );
        assert_eq!(resource1.add(resource4).unwrap(), resource1);
        assert_eq!(resource4.add(resource5).unwrap(), resource4);
    }

    #[test]
    fn test_is_empty() {
        let resource1 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 10, 100);
        let resource2 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 0, 100);

        assert_eq!(resource1.is_empty(), false);
        assert_eq!(resource2.is_empty(), true);
    }

    #[test]
    fn test_from_type_amount_and_max_amount() {
        let resource1 =
            MineableResource::from_type_amount_and_max_amount(MineableResourceType::COAL, 10, 100);

        assert_eq!(resource1.resource_type, MineableResourceType::COAL);
        assert_eq!(resource1.current_amount, 10);
        assert_eq!(resource1.max_amount, 100);
    }
}
