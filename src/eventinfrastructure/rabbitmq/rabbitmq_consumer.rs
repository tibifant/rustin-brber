use std::collections::HashMap;
use amqprs::{BasicProperties, Deliver, FieldValue};
use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use async_trait::async_trait;
use tracing::info;
use super::super::game_event_header::GameEventHeader;

pub struct RabbitMQConsumer {
    no_ack: bool,
}

impl RabbitMQConsumer {
    pub fn new(no_ack: bool) -> Self {
        Self { no_ack }
    }

    fn parse_header(&self, properties: BasicProperties) -> Result<GameEventHeader, String> {
        let headers = properties.headers().unwrap();
        let mut header_map: HashMap<&str, String> = HashMap::new();

        // Convert headers to a HashMap
        for (key, value) in headers.as_ref() {
            header_map.insert(key.as_ref().as_ref(), self.extract_string_from_byte_array(value)?);
        }

        // Initialize and populate GameEventHeader from the HashMap
        let header = GameEventHeader {
            event_id: header_map.get("eventId").unwrap().to_owned(),
            version: header_map.get("version").unwrap().to_owned(),
            player_id: header_map.get("playerId").unwrap().to_owned(),
            timestamp: header_map.get("timestamp").unwrap().to_owned(),
            transaction_id: header_map.get("transactionId").unwrap().to_owned(),
            event_type: header_map.get("type").unwrap().to_owned(),
            kafka_topic: header_map.get("kafka-topic").unwrap().to_owned(),
        };

        info!("Game Event Header {:?}", &header);

        Ok(header)
    }


    fn extract_string_from_byte_array(&self, value: &FieldValue) -> Result<String, String> {
        if let FieldValue::x(byte_array) = value {
            String::from_utf8(byte_array.clone().into()).map_err(|e| e.to_string())
        } else {
            Err("Expected a ByteArray".to_string())
        }
    }
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