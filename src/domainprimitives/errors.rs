use thiserror::Error;

use crate::domainprimitives::location::mineable_resource_type::MineableResourceType;

#[derive(Error, Debug, PartialEq)]
pub enum DomainPrimitiveError {
    #[error("Resource Type {0} does not match {1} for addition")]
    InvalidResourceType(MineableResourceType, MineableResourceType),
    #[error("Energy was tried to be reduced from {0} to {1}")]
    NegativeEnergy(u16, u16),
    #[error("Money was tried to be reduced from {0} to {1}")]
    NegativeMoney(u64, u64),
}
