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