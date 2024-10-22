use std::sync::Arc;

use async_trait::async_trait;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::robot::robots_revealed_event::RobotsRevealedEvent;
use crate::robot::application::robot_application_service::RobotApplicationService;

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
        self.robot_application_service.add_or_update_robots(&r.robot_id, &r.planet_id).await;
      }
    }
}
