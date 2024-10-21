use std::sync::Arc;

use async_trait::async_trait;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::robot::robot_spawned_event::RobotSpawnedEvent;
use crate::game::application::game_application_service::GameApplicationService;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct RobotSpawnedEventHandler {
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    game_application_service: Arc<GameApplicationService>,
}

impl RobotSpawnedEventHandler {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        game_application_service: Arc<GameApplicationService>,
    ) -> Self {
        Self {
            game_service_rest_adapter,
            game_application_service,
        }
    }
}

#[async_trait]
impl EventHandler<RobotSpawnedEvent> for RobotSpawnedEventHandler {
    async fn handle(&self, event: RobotSpawnedEvent) {
        self.game_application_service.robot_spawned(&event.robot.robot_id, &event.robot.planet.planet_id).await;
    }
}
