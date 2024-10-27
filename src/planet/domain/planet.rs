use crate::domainprimitives::location::mineable_resource::MineableResource;

#[derive(Clone)]
pub struct TransientPlanetInfo {
  pub id: String,
  pub resource: Option<MineableResource>,
}

impl TransientPlanetInfo {
  pub fn new(id: String, resource: Option<MineableResource>) -> Self {
    Self {
      id,
      resource,
    }
  }
}

#[derive(Clone)]
pub struct PersistentPlanetInfo {
  pub id: String,
  pub movement_difficulty: u8,
  pub resource: Option<MineableResource>,
  pub north: String,
  pub east: String,
  pub west: String,
  pub south: String,                  
}

impl PersistentPlanetInfo {
  pub fn new(id: String, movement_difficulty: u8, resource: Option<MineableResource>, north: String, east: String, west: String, south: String) -> Self {
    Self {
      id,
      movement_difficulty,
      resource,
      north,
      east,
      west,
      south,
    }
  }
}