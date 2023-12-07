use tracing::{error, info};
use crate::player::player::Player;
use crate::rest::game_service_rest_adapter::*;

use crate::eventinfrastructure::rabbitmq::rabbitmq_connection_handler::RabbitMQConnectionHandler;
use crate::rest::request::command::command::Command;

pub struct DungeonPlayerStartupHandler {
    player: Player,
    game_service_rest_adapter: GameServiceRESTAdapter,
    rabbitmq_connection_handler: RabbitMQConnectionHandler,
}

impl DungeonPlayerStartupHandler {
    pub async fn new() -> Self {
        Self {
            player: Player::new(),
            game_service_rest_adapter: GameServiceRESTAdapter::new(),
            rabbitmq_connection_handler: RabbitMQConnectionHandler::new().await.map_err(|e| error!("Failed to create RabbitMQConnectionHandler {e}")).unwrap(),
        }
    }

    pub async fn register_and_listen_for_events(&mut self) {
        self.register_player().await;
        self.rabbitmq_connection_handler.listen_for_events(&self.player).await;
        let games = self.game_service_rest_adapter.get_joinable_games().await.expect("Error happend during fetching games");
        self.game_service_rest_adapter.join_game(&games[0].game_id).await;
        //self.game_service_rest_adapter.send_command(Command::create_robot_purchase_command(self.player.player_id.to_string(), 5)).await.expect("TODO: panic message");
    }
    async fn register_player(&mut self) {
        let player_response = self.game_service_rest_adapter.register_player().await.map_err(|e| error!("Failed to register player {e}")).unwrap();
        self.player = player_response;
        info!("Registered player: {:?}", &self.player);
    }
}