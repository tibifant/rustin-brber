use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver, FieldValue, ShortStr};
use async_trait::async_trait;
use serde_json::{json, Value};
use tracing::{error, info};

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
        Self {
            no_ack,
            event_dispatcher,
        }
    }

    fn parse_header(&self, properties: BasicProperties) -> Result<GameEventHeader, ParseError> {
        let headers = properties.headers().ok_or_else(|| {
            ParseError::MissingField("Headers were non-existent in properties!".to_string())
        })?;

        let fetch_field = |field: &str| -> Option<String> {
            let field_name = ShortStr::try_from(field).ok();
            match field_name {
                Some(name) => {
                    headers
                        .get(&name)
                        .ok_or_else(|| {
                            ParseError::MissingField(format!(
                                "Header with key {} was not found in the FieldTable!",
                                field.to_string()
                            ))
                        })
                        .and_then(|field_value| self.extract_string_from_byte_array(field_value))
                        .ok()
                },
                None => None,
            }
        };

        Ok(GameEventHeader {
            event_id: fetch_field("eventId"),
            version: fetch_field("version"),
            player_id: fetch_field("playerId"),
            timestamp: fetch_field("timestamp"),
            transaction_id: fetch_field("transactionId"),
            event_type: fetch_field("type"),
            kafka_topic: fetch_field("kafka-topic"),
        })
    }

    fn extract_string_from_byte_array(&self, value: &FieldValue) -> Result<String, ParseError> {
        if let FieldValue::x(byte_array) = value {
            String::from_utf8(byte_array.clone().into()).or_else(|e| {
                Err(ParseError::InvalidType(format!(
                    "Could not parse byte array to string: {}",
                    e
                )))
            })
        } else {
            Err(ParseError::InvalidType(format!(
                "Expected a ByteArray as type of header value but was: {:?}",
                value
            )))
        }
    }

    async fn handle_event(&mut self, game_event: GameEvent) {
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
        let header = match self.parse_header(_basic_properties.clone()) {
            Ok(header) => header,
            Err(e) => {
                error!("Error parsing header: {}", e);
                //println!("Error with header: {:?}", std::str::from_utf8(_basic_properties.headers));
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
            Err(e) => {
                let error: ParseError = ParseError::InvalidType(format!(
                    "{:?}\nerror: `{}` in {}:{}\n====================================================\n{}",
                    header.event_type,
                    e.to_string(),
                    e.line(),
                    e.column(),
                    serde_json::to_string_pretty(&body_json)
                        .expect("Could not serialize body to string")
                ));
                error!("{}\n", error);
                return;
            }
        };
        info!("EVENT TYPE: {:?}", game_event_type);
        let game_event = GameEvent {
            header,
            event_body: game_event_type,
        };
        //info!("Received event: {:?}", game_event);
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
    
    use tokio::sync::Mutex;

    use amqprs::{FieldName, FieldTable};

    use crate::eventinfrastructure::event_dispatcher::EventDispatcher;
    use crate::game::application::game_application_service::GameApplicationService;
    use crate::game::application::game_logic_service::GameLogicService;
    use crate::player::application::player_application_service::PlayerApplicationService;
    use crate::rest::game_service_rest_adapter_impl::{self, GameServiceRestAdapterImpl};

    use super::*;

    fn get_rabbitmq_consumer() -> RabbitMQConsumer {
        let game_service_rest_adapter = Arc::new(GameServiceRestAdapterImpl::new());
        let game_logic = Arc::new(Mutex::new(GameLogicService::new()));
        let player_application_service = Arc::new(PlayerApplicationService::new(
            game_service_rest_adapter.clone(),
            game_logic.clone()
            ));
        let game_application_service = Arc::new(GameApplicationService::new(
            game_service_rest_adapter.clone(),
            game_logic.clone()
            ));

        RabbitMQConsumer::new(
            false,
            EventDispatcher::new(
                Arc::new(GameServiceRestAdapterImpl::new()),
                game_application_service,
                player_application_service,
                game_logic.clone(),
            ),
        )
    }

    #[test]
    fn test_parse_header() {
        let mut headers = FieldTable::new();

        headers.insert(
            FieldName::try_from("eventId").unwrap(),
            FieldValue::x("eventId".as_bytes().to_vec().try_into().unwrap()),
        );
        headers.insert(
            FieldName::try_from("version").unwrap(),
            FieldValue::x("version".as_bytes().to_vec().try_into().unwrap()),
        );
        headers.insert(
            FieldName::try_from("playerId").unwrap(),
            FieldValue::x("playerId".as_bytes().to_vec().try_into().unwrap()),
        );
        headers.insert(
            FieldName::try_from("timestamp").unwrap(),
            FieldValue::x("timestamp".as_bytes().to_vec().try_into().unwrap()),
        );
        headers.insert(
            FieldName::try_from("transactionId").unwrap(),
            FieldValue::x("transactionId".as_bytes().to_vec().try_into().unwrap()),
        );
        headers.insert(
            FieldName::try_from("type").unwrap(),
            FieldValue::x("type".as_bytes().to_vec().try_into().unwrap()),
        );
        headers.insert(
            FieldName::try_from("kafka-topic").unwrap(),
            FieldValue::x("kafka-topic".as_bytes().to_vec().try_into().unwrap()),
        );
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
}
