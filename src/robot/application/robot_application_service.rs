use std::sync::Arc;

use tracing::{error, info};

use crate::player::application::player_application_service::PlayerApplicationService;
use crate::robot::domain::robot::Robot;
use crate::repository::AsyncRepository;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;
use crate::domainprimitives::command::command::Command;
use crate::repository::InMemoryRepository;
use crate::repository::Identifiable;

pub struct RobotApplicationService {
    robot_repository: Box<dyn AsyncRepository<Robot> + Send + Sync>,
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    player_application_service: Arc<PlayerApplicationService>,
}

impl RobotApplicationService {
    pub fn new(
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
        player_application_service: Arc<PlayerApplicationService>,
    ) -> Self {
        let robot_repository = Box::new(InMemoryRepository::new());
        Self {
            robot_repository,
            game_service_rest_adapter,
            player_application_service,
        }
    }

    pub async fn buy_robots(&self) {
        let player = self.player_application_service.query_and_if_needed_create_player().await;
        let buy_robot_command = Command::create_robot_purchase_command(player.id(), 1);
        info!("====> Try to buy 1 Robot!!!!!!!!!!!!!!.");
        let command_info_repsonse = self.game_service_rest_adapter.send_command(buy_robot_command).await;
        info!("------ {:?}", command_info_repsonse);
    }

    pub async fn add_robot(&self, robot_id: &str, planet_id: &str) {
        let robot = Robot::new(robot_id.to_string(), planet_id.to_string());
        let _ = self.robot_repository.add(robot);
        info!("====> added robot -------!!!!!!!!");
    }

    pub async fn add_or_update_robots(&self, robot_id: &str, planet_id: &str) {
        info!("====> robot revealed -------!!!!!!!!");
    }
}
