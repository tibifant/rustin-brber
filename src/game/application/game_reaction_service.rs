use std::sync::Arc;

use tracing::{error, info};

use crate::player::application::player_application_service::{self, PlayerApplicationService};
use crate::rest::game_service_rest_adapter_impl;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;
use crate::robot::application::robot_application_service::{self, RobotApplicationService};

pub struct GameReactionService {
  game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
  player_application_service: Arc<PlayerApplicationService>,
  robot_application_service: Arc<RobotApplicationService>,
}

impl GameReactionService {
  pub fn new(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>, 
    player_application_service: Arc<PlayerApplicationService>,
    robot_application_service: Arc<RobotApplicationService>,
  ) -> Self {
      Self {
        game_service_rest_adapter,
        player_application_service,
        robot_application_service,
      }
  }

  pub async fn decide(&self) {
    self.robot_application_service.buy_robots().await;
  }
}