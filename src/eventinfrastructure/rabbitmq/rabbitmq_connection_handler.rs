use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, Channel, QueuePurgeArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};

use crate::config::CONFIG;
use crate::eventinfrastructure::event_dispatcher::EventDispatcher;
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
        let connection = Connection::open(&connection_arguments)
            .await
            .map_err(|_| RabbitMQConnectionError::FailedToOpenConnection)?;
        connection
            .register_callback(DefaultConnectionCallback)
            .await
            .map_err(|_| RabbitMQConnectionError::FailedToRegisterCallback)?;
        let channel = connection
            .open_channel(None)
            .await
            .map_err(|_| RabbitMQConnectionError::FailedToOpenChannel)?;
        channel
            .register_callback(DefaultChannelCallback)
            .await
            .map_err(|_| RabbitMQConnectionError::FailedToRegisterCallbackForChannel)?;
        Ok(Self {
            connection,
            channel,
        })
    }

    pub async fn purge_queue(&self, queue_name: &str) {
        self.channel
            .queue_purge(QueuePurgeArguments::new(queue_name))
            .await
            .expect("Failed to purge queue");
    }
    pub async fn listen_for_and_handle_events(&self, player: &Player, event_dispatcher: EventDispatcher) {
        self.channel
            .basic_consume(
                RabbitMQConsumer::new(false, event_dispatcher),
                BasicConsumeArguments::new(
                    &player.player_queue,
                    format!("{}-CONSUMER", CONFIG.player_name).as_str(),
                ),
            )
            .await
            .expect("Failed to consume Events");
    }
}
