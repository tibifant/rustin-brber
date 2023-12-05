use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, Channel};
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::config::CONFIG;
use crate::player::player::Player;
use super::rabbitmq_consumer::RabbitMQConsumer;

pub struct RabbitMQConnectionHandler {
    connection: Connection,
    channel: Channel,
}



impl RabbitMQConnectionHandler {
    pub async fn new() -> Self {
        let connection_arguments = OpenConnectionArguments::new(
            &CONFIG.rabbitmq_host,
            CONFIG.rabbitmq_port.clone(),
            &CONFIG.rabbitmq_username,
            &CONFIG.rabbitmq_password,
        );
        let connection = Connection::open(&connection_arguments).await.expect("Failed to open connection");
        connection.register_callback(DefaultConnectionCallback).await.expect("Failed to register callback");
        let channel = connection.open_channel(None).await.expect("Failed to open channel");
        channel.register_callback(DefaultChannelCallback).await.expect("Failed to register callback for channel");
        Self {
            connection,
            channel,
        }
    }
    pub async fn listen_for_events(&self, player: &Player) {
        self.channel
            .basic_consume(
                RabbitMQConsumer::new(false),
                BasicConsumeArguments::new(
                    &player.player_queue,
                    "RustDungeonPlayerConsumer",
                ),
            )
            .await
            .expect("Failed to consume Events");
    }
}

