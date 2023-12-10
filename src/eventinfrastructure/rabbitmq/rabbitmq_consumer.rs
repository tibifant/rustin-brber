

use amqprs::{BasicProperties, Deliver, FieldValue, ShortStr};
use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use async_trait::async_trait;
use serde_json::{json, Value};
use tracing::error;
use crate::eventinfrastructure::event_dispatcher::EventDispatcher;
use crate::eventinfrastructure::game_event::GameEvent;
use crate::eventinfrastructure::game_event_body_type::GameEventBodyType;
use crate::eventinfrastructure::rabbitmq::errors::ParseError;

use super::super::game_event_header::GameEventHeader;

pub struct RabbitMQConsumer {
    no_ack: bool,
    event_dispatcher: EventDispatcher,
}


impl RabbitMQConsumer {
    pub fn new(no_ack: bool, event_dispatcher: EventDispatcher) -> Self {
        Self { no_ack, event_dispatcher }
    }

    fn parse_header(&self, properties: BasicProperties) -> Result<GameEventHeader, ParseError> {
        let headers = properties.headers().ok_or_else(|| ParseError::MissingField("Headers were non-existent in properties!".to_string()))?;

        let fetch_field = |field: &str| {
            let field_name = ShortStr::try_from(field)
                .map_err(|_| ParseError::InvalidType(format!("Fieldname '{}' was not able to be parsed into a ShortStr", field)))?;
            headers.get(&field_name)
                .ok_or_else(|| ParseError::MissingField(format!("Header with key {} was not found in the FieldTable!", field.to_string())))
                .and_then(|field_value| self.extract_string_from_byte_array(field_value))
        };

        Ok(GameEventHeader {
            event_id: Some(fetch_field("eventId")?),
            version: Some(fetch_field("version")?),
            player_id: Some(fetch_field("playerId")?),
            timestamp: Some(fetch_field("timestamp")?),
            transaction_id: Some(fetch_field("transactionId")?),
            event_type: Some(fetch_field("type")?),
            kafka_topic: Some(fetch_field("kafka-topic")?),
        })
    }


    fn extract_string_from_byte_array(&self, value: &FieldValue) -> Result<String, ParseError> {
        if let FieldValue::x(byte_array) = value {
            String::from_utf8(byte_array.clone().into()).or_else(|e| Err(ParseError::InvalidType(format!("Could not parse byte array to string: {}", e))))
        } else {
            Err(ParseError::InvalidType(format!("Expected a ByteArray as type of header value but was: {:?}", value)))
        }
    }

    async fn handle_event(&self, game_event: GameEvent) {
        self.event_dispatcher.dispatch(game_event).await;
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
        let header = match self.parse_header(_basic_properties) {
            Ok(header) => header,
            Err(e) => {
                error!("Error parsing header: {}", e);
                return;
            }
        };
        let body_json: Value = match serde_json::from_slice(&content) {
            Ok(json) => json,
            Err(_) => {
                error!("Unexpected Error parsing body. This should not happen because the Game Service should always send valid Json through RabbitMQ");
                return;
            }
        };
        let game_event_json = json!({
            "type": header.event_type,
            "event": body_json
        });

        let game_event_type: GameEventBodyType = match serde_json::from_value(game_event_json) {
            Ok(game_event) => game_event,
            Err(_) => {
                let error: ParseError = ParseError::InvalidType(format!("{:?}\n{}", header.event_type, serde_json::to_string_pretty(&body_json).expect("Could not serialize body to string")));
                error!("{}", error);
                return;
            }
        };
        let game_event = GameEvent {
            header,
            event_body: game_event_type,
        };
        self.handle_event(game_event).await;
        if !self.no_ack {
            #[cfg(feature = "traces")]
            info!("ack to delivery {} on channel {}", deliver, channel);
            let args = BasicAckArguments::new(deliver.delivery_tag(), true);
            channel.basic_ack(args).await.expect("basic_ack");
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use amqprs::{FieldName, FieldTable};

    use crate::eventinfrastructure::event_dispatcher::EventDispatcher;
    use crate::rest::game_service_rest_adapter_impl::GameServiceRestAdapterImpl;

    use super::*;

    fn get_rabbitmq_consumer() -> RabbitMQConsumer {
        RabbitMQConsumer::new(false, EventDispatcher::new(Arc::new(GameServiceRestAdapterImpl::new())))
    }

    #[test]
    fn test_parse_header() {
        let mut headers = FieldTable::new();

        headers.insert(FieldName::try_from("eventId").unwrap(), FieldValue::x("eventId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("version").unwrap(), FieldValue::x("version".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("playerId").unwrap(), FieldValue::x("playerId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("timestamp").unwrap(), FieldValue::x("timestamp".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("transactionId").unwrap(), FieldValue::x("transactionId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("type").unwrap(), FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("kafka-topic").unwrap(), FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()));
        let properties = BasicProperties::default().with_headers(headers).to_owned();
        let consumer = get_rabbitmq_consumer();
        let header = consumer.parse_header(properties).unwrap();
        assert_eq!(header.event_id.unwrap(), "eventId");
        assert_eq!(header.version.unwrap(), "version");
        assert_eq!(header.player_id.unwrap(), "playerId");
        assert_eq!(header.timestamp.unwrap(), "timestamp");
        assert_eq!(header.transaction_id.unwrap(), "transactionId");
        assert_eq!(header.event_type.unwrap(), "type");
        assert_eq!(header.kafka_topic.unwrap(), "kafka-topic");
    }

    #[test]
    fn test_parse_header_missing_event_id() {
        let mut headers = FieldTable::new();

        headers.insert(FieldName::try_from("version").unwrap(), FieldValue::x("version".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("playerId").unwrap(), FieldValue::x("playerId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("timestamp").unwrap(), FieldValue::x("timestamp".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("transactionId").unwrap(), FieldValue::x("transactionId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("type").unwrap(), FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("kafka-topic").unwrap(), FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()));
        let properties = BasicProperties::default().with_headers(headers).to_owned();
        let consumer = get_rabbitmq_consumer();
        let header = consumer.parse_header(properties);
        assert!(header.is_err());
        match header.unwrap_err() {
            ParseError::MissingField(field) => assert!(true),
            other => assert!(false, "Expected ParseError::MissingField but got {:?}", other),
        }
    }

    #[test]
    fn test_parse_header_missing_version() {
        let mut headers = FieldTable::new();

        headers.insert(FieldName::try_from("eventId").unwrap(), FieldValue::x("eventId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("playerId").unwrap(), FieldValue::x("playerId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("timestamp").unwrap(), FieldValue::x("timestamp".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("transactionId").unwrap(), FieldValue::x("transactionId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("type").unwrap(), FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("kafka-topic").unwrap(), FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()));
        let properties = BasicProperties::default().with_headers(headers).to_owned();
        let consumer = get_rabbitmq_consumer();
        let header = consumer.parse_header(properties);
        assert!(header.is_err());
        match header {
            Err(ParseError::MissingField(field)) => assert!(true),
            other => assert!(false, "Expected ParseError::MissingField but got {:?}", other),
        }
    }

    #[test]
    fn test_parse_header_missing_player_id() {
        let mut headers = FieldTable::new();

        headers.insert(FieldName::try_from("eventId").unwrap(), FieldValue::x("eventId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("version").unwrap(), FieldValue::x("version".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("timestamp").unwrap(), FieldValue::x("timestamp".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("transactionId").unwrap(), FieldValue::x("transactionId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("type").unwrap(), FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("kafka-topic").unwrap(), FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()));
        let properties = BasicProperties::default().with_headers(headers).to_owned();
        let consumer = get_rabbitmq_consumer();
        let header = consumer.parse_header(properties);
        assert!(header.is_err());
        match header {
            Err(ParseError::MissingField(field)) => assert!(true),
            other => assert!(false, "Expected ParseError::MissingField but got {:?}", other),
        }
    }

    #[test]
    fn test_parse_header_missing_timestamp() {
        let mut headers = FieldTable::new();

        headers.insert(FieldName::try_from("eventId").unwrap(), FieldValue::x("eventId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("version").unwrap(), FieldValue::x("version".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("playerId").unwrap(), FieldValue::x("playerId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("transactionId").unwrap(), FieldValue::x("transactionId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("type").unwrap(), FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("kafka-topic").unwrap(), FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()));
        let properties = BasicProperties::default().with_headers(headers).to_owned();
        let consumer = get_rabbitmq_consumer();
        let header = consumer.parse_header(properties);
        assert!(header.is_err());
        match header {
            Err(ParseError::MissingField(field)) => assert!(true),
            other => assert!(false, "Expected ParseError::MissingField but got {:?}", other),
        }
    }

    #[test]
    fn test_parse_header_missing_transaction_id() {
        let mut headers = FieldTable::new();

        headers.insert(FieldName::try_from("eventId").unwrap(), FieldValue::x("eventId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("version").unwrap(), FieldValue::x("version".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("playerId").unwrap(), FieldValue::x("playerId".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("timestamp").unwrap(), FieldValue::x("timestamp".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("type").unwrap(), FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()));
        headers.insert(FieldName::try_from("kafka-topic").unwrap(), FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()));
        let properties = BasicProperties::default().with_headers(headers).to_owned();
        let consumer = get_rabbitmq_consumer();
        let header = consumer.parse_header(properties);
        assert!(header.is_err());
        match header {
            Err(ParseError::MissingField(field)) => assert!(true),
            other => assert!(false, "Expected ParseError::MissingField but got {:?}", other),
        }
    }
}