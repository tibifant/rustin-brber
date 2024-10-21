use std::sync::Arc;

use async_trait::async_trait;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::robot::robot_spawned_event::RobotSpawnedEvent;
use crate::robot::application::robot_application_service::RobotApplicationService;

pub struct RobotSpawnedEventHandler {
    robot_application_service: Arc<RobotApplicationService>,
}

impl RobotSpawnedEventHandler {
    pub fn new(robot_application_service: Arc<RobotApplicationService>) -> Self {
        Self {
            robot_application_service
        }
    }
}

#[async_trait]
impl EventHandler<RobotSpawnedEvent> for RobotSpawnedEventHandler {
    async fn handle(&self, event: RobotSpawnedEvent) {
        self.robot_application_service.add_robot(&event.robot.robot_id, &event.robot.planet.planet_id).await;
    }
}
