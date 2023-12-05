use tracing::{info};
use crate::player::player::Player;
use crate::rest::game_service_rest_adapter::*;

use crate::rabbitmq::rabbitmq_connection_handler::RabbitMQConnectionHandler;

pub(crate) struct DungeonPlayerStartupHandler {
    player: Option<Player>,
    game_service_rest_adapter: GameServiceRESTAdapter,
    rabbitmq_connection_handler: RabbitMQConnectionHandler,
}

impl DungeonPlayerStartupHandler {
    pub async fn new() -> Self {
        Self {
            player: None,
            game_service_rest_adapter: GameServiceRESTAdapter::new(),
            rabbitmq_connection_handler: RabbitMQConnectionHandler::new().await,
        }
    }
    pub async fn register_and_listen_for_events(&mut self) {
        self.register_player().await;
        self.rabbitmq_connection_handler.listen_for_events(self.player.as_ref().unwrap()).await;
    }
    fn initialize_player(&mut self, player: Player) {
        self.player = Some(player);
    }
    async fn register_player(&mut self) {
        let player_response = self.game_service_rest_adapter.register_player().await.expect("Failed to register player");
        self.initialize_player(player_response);
        info!("Registered player: {:?}", &self.player);
    }
}