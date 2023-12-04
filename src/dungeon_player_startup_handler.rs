use tracing::info;
use crate::player::player::Player;
use crate::rest::game_service_rest_adapter::*;

use lapin::{Connection, ConnectionProperties, Consumer, Result};
use crate::config::CONFIG;

pub(crate) struct DungeonPlayerStartupHandler {
    player: Option<Player>,
    game_service_rest_adapter: GameServiceRESTAdapter,
}

impl DungeonPlayerStartupHandler {
    pub fn new() -> Self {
        Self {
            player: None,
            game_service_rest_adapter: GameServiceRESTAdapter::new(),
        }
    }
    fn initialize_player(&mut self, player: Player) {
        self.player = Some(player);
    }
    pub async fn register_player(&mut self) {
        let player_response = self.game_service_rest_adapter.register_player().await.expect("Failed to register player");
        self.initialize_player(player_response);
        info!("Registered player: {:?}", &self.player);
    }

    async fn listen_for_events(&self) {
        let uri = format!("amqp://{}:{}@{}:{}",
                          CONFIG.rabbitmq_username.clone(),
                          CONFIG.rabbitmq_password.clone(),
                          CONFIG.rabbitmq_host.clone(),
                          CONFIG.rabbitmq_port.clone());
        let connection = Connection::connect(
            &uri,
            ConnectionProperties::default()
                .with_executor(tokio_executor_trait::Tokio::current()),
        ).await.unwrap();
        let channel = connection.create_channel().await.unwrap();
        channel.basic_consume(
            &self.player.as_ref().unwrap().player_queue,
            "RustSkeletonConsumer",
            lapin::options::BasicConsumeOptions::default(),
            lapin::types::FieldTable::default(),
        ).await.expect("Failed to consume messages");
    }

    async fn handle_messages(&self, mut consumer: Consumer) {}
}