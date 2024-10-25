use std::sync::Arc;

use tracing::{error, info};

use crate::player::application::player_application_service::PlayerApplicationService;
use crate::robot::domain::robot::MinimalRobot;
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

    pub async fn add_robot(&self, player_id: &str, robot: Robot) {
        let own_player_id = self.player_application_service.query_and_if_needed_create_player().await.player_id;

        match own_player_id {
            Some(own_player_id) if own_player_id == player_id => {
                let _ = self.robot_repository.add(robot.clone());
                info!("====> added robot -------!!!!!!!!\nwith id: {}", robot.robot_info.id);
                info!("robot id {}", robot.id());
                let robot_result = self.robot_repository.get(&robot.robot_info.id);
                info!("!");
            }
            Some(_) => { 

            }
            None => {
                println!("====> player is none!")
            }
        }
    }

    pub async fn robots_revealed(&self, player_notion: &str, robot_info: MinimalRobot) {
        info!("====> robot revealed -------!!!!!!!!");

        let own_player_id = self.player_application_service.query_and_if_needed_create_player().await.player_id;

        match own_player_id {
            Some(own_player_id) if own_player_id.starts_with(player_notion) => { // if robot belongs to us
                let r = self.robot_repository.get(&robot_info.id);
                let robot_result = self.robot_repository.get(&robot_info.id).await;
                //match robot_result {
                //    Ok(Some(mut robot)) => {
                //        if !robot.check_health(robot_info.clone()) { // returns false if health is 0
                //            let _ = self.robot_repository.delete(robot.id().as_str()).await;
                //        }
                //        else {
                //            robot.update(robot_info);
                //        }
                //    }
                //    Ok(None) => {
                //        info!("Robot not saved yet. Id: {}", robot_info.robot_id);
                //    }
                //    Err(e) => {
                //        println!("Error occurred whilst trying to get Robot: {}", e);
                //    }
                //}
                
            }
            Some(_) => { // if robot belongs to someone else
                // TODO: check planet and save robot there
            }
            None => {
                println!("====> player is none!")
            }
        }
    }
}
