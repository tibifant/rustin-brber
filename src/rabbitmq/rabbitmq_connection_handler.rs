use std::collections::HashMap;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicAckArguments, BasicConsumeArguments, Channel};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver, FieldValue};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{info};
use crate::config::CONFIG;
use crate::player::player::Player;

pub struct RabbitMQConnectionHandler {
    connection: Connection,
    channel: Channel,
}

struct RabbitMQConsumer {
    no_ack: bool,
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

impl RabbitMQConsumer {
    pub fn new(no_ack: bool) -> Self {
        Self { no_ack }
    }

    pub fn parse_header(&self, properties: BasicProperties) -> Result<GameEventHeader, String> {
        let headers = properties.headers().unwrap();
        let mut header_map = HashMap::new();

        // Convert headers to a HashMap
        for (key, value) in headers.as_ref() {
            header_map.insert(key.as_ref().as_ref(), self.extract_string_from_byte_array(value)?);
        }

        // Initialize and populate GameEventHeader from the HashMap
        let header = GameEventHeader {
            event_id: header_map.get("eventId").cloned().unwrap_or_default(),
            version: header_map.get("version").cloned().unwrap_or_default(),
            player_id: header_map.get("playerId").cloned().unwrap_or_default(),
            timestamp: header_map.get("timestamp").cloned().unwrap_or_default(),
            transaction_id: header_map.get("transactionId").cloned().unwrap_or_default(),
            event_type: header_map.get("eventType").cloned().unwrap_or_default(),
            kafka_topic: header_map.get("kafka-topic").cloned().unwrap_or_default(),
        };

        info!("Game Event Header {:?}", &header);

        Ok(header)
    }


    pub fn extract_string_from_byte_array(&self, value: &FieldValue) -> Result<String, String> {
        if let FieldValue::x(byte_array) = value {
            String::from_utf8(byte_array.clone().into()).map_err(|e| e.to_string())
        } else {
            Err("Expected a ByteArray".to_string())
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GameEventHeader {
    event_id: String,
    version: String,
    player_id: String,
    timestamp: String,
    transaction_id: String,
    event_type: String,
    #[serde(rename = "kafka-topic")]
    kafka_topic: String,
}

#[async_trait]
impl AsyncConsumer for RabbitMQConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        info!(
            "consume delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );
        self.parse_header(_basic_properties).expect("Unexpected Error parsing header");



        if !self.no_ack {
            #[cfg(feature = "traces")]
            info!("ack to delivery {} on channel {}", deliver, channel);
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}
