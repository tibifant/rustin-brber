use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, Channel};
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::config::CONFIG;
use crate::eventinfrastructure::rabbitmq::errors::RabbitMQConnectionError;
use crate::player::player::Player;
use super::rabbitmq_consumer::RabbitMQConsumer;

pub struct RabbitMQConnectionHandler {
    connection: Connection,
    channel: Channel,
}


impl RabbitMQConnectionHandler {
    pub async fn new() -> Result<Self, RabbitMQConnectionError> {
        let connection_arguments = OpenConnectionArguments::new(
            &CONFIG.rabbitmq_host,
            CONFIG.rabbitmq_port.clone(),
            &CONFIG.rabbitmq_username,
            &CONFIG.rabbitmq_password,
        );
        let connection = Connection::open(&connection_arguments).await.map_err(|_| RabbitMQConnectionError::FailedToOpenConnection)?;
        connection.register_callback(DefaultConnectionCallback).await.map_err(|_| RabbitMQConnectionError::FailedToRegisterCallback)?;
        let channel = connection.open_channel(None).await.map_err(|_| RabbitMQConnectionError::FailedToOpenChannel)?;
        channel.register_callback(DefaultChannelCallback).await.map_err(|_| RabbitMQConnectionError::FailedToRegisterCallbackForChannel)?;
        Ok(Self {
            connection,
            channel,
        })
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

