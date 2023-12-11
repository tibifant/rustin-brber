use serde::{Deserialize, Serialize};
use crate::domainprimitives::location::mineable_resource::MineableResource;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanetResourceMinedEvent {
    pub planet_id: String,
    pub mined_amount: u32,
    pub resource: MineableResource
}