use serde::{Deserialize, Serialize};

use crate::config::CONFIG;
use crate::eventinfrastructure::map::planet_discovered_event;
use crate::repository::Identifiable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Robot {
  robot_id: String,
  planet_id: String,
}

impl Robot {
  pub fn new(robot_id: String, planet_id: String) -> Self {
    Self {
      robot_id,
      planet_id,
    }
  }

  pub fn change_planet(&mut self, new_planet_id: String) {
    self.planet_id = new_planet_id;
  }
}

impl Identifiable for Robot {
    fn id(&self) -> String {
        return self.robot_id.clone();
    }
}