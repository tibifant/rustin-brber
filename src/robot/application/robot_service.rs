use std::sync::Arc;

use tracing::{error, info};

use crate::robot::domain::robot::Robot;
use crate::repository::AsyncRepository;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

pub struct RobotApplicationService {
    robot_repository: Box<dyn AsyncRepository<Player> + Send + Sync>,
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
}

impl PlayerApplicationService {
    pub fn new(
        robot_repository: Box<dyn AsyncRepository<Player> + Send + Sync>,
        game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
    ) -> Self {
        Self {
            robot_repository,
            game_service_rest_adapter,
        }
    }

    pub fn buy_robots(round_status_event: RoundStatusEvent) {
      if round_status_event.round_status == RoundStatusDto::Started {
        let player = player_application_service.query_and_if_needed_create_player();
        let buy_robot_command = Command.create_robot_purchase_command(player.id, 1);
        game_service_rest_adapter.send_command(buy_robot_command);
        info!("Try to buy 1 Robot.")
      }
    }
}
