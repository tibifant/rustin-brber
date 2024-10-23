use std::sync::Arc;

use async_trait::async_trait;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::robot::robots_revealed_event::RobotsRevealedEvent;
use crate::robot::application::robot_application_service::RobotApplicationService;
use crate::robot::domain::robot::{MinimalRobot, Robot};

pub struct RobotsRevealedEventHandler {
    robot_application_service: Arc<RobotApplicationService>,
}

impl RobotsRevealedEventHandler {
    pub fn new(robot_application_service: Arc<RobotApplicationService>) -> Self {
        Self {
            robot_application_service,
        }
    }
}

#[async_trait]
impl EventHandler<RobotsRevealedEvent> for RobotsRevealedEventHandler {
    async fn handle(&self, event: RobotsRevealedEvent) {
      for r in event.robots.iter() {
        let robot = MinimalRobot::new(r.robot_id.to_string(), r.planet_id.to_string(), r.energy, r.health, r.levels.health_level, r.levels.damage_level, r.levels.mining_speed_level, r.levels.mining_level, r.levels.energy_level, r.levels.energy_regen_level, r.levels.storage_level);
        self.robot_application_service.robots_revealed(&r.player_notion, robot).await;
      }
    }
}
