use serde::{Deserialize, Serialize};

use crate::config::CONFIG;
use crate::repository::Identifiable;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Robot {
  robot_id: Option<String>,
}

impl Robot {
  pub fn new() -> Self {
    Self {
      robot_id: None,
    }
  }

  pub fn assign_robot_id(&mut self, robot_id: String) {
    self.robot_id = Some(robot_id);
  }
}

impl Identifiable for Robot {
    fn id(&self) -> String {
        self.robot_id.clone().expect("Robot id is not set")
    }
}