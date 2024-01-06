use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info};

use crate::domainprimitives::command::command::Command;
use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::game::game_status_event::GameStatusEvent;
use crate::game::domain::game_status::GameStatus;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;

#[derive(Debug)]
pub struct GameStatusEventHandler {
    game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>,
}

impl GameStatusEventHandler {
    pub fn new(game_service_rest_adapter: Arc<dyn GameServiceRestAdapterTrait>) -> Self {
        Self {
            game_service_rest_adapter,
        }
    }
}

#[async_trait]
impl EventHandler<GameStatusEvent> for GameStatusEventHandler {
    async fn handle(&self, event: GameStatusEvent) {
        match event.status {
            GameStatus::CREATED => {
                info!("Game {} Status: Created", event.game_id);
                let join_game_result = self
                    .game_service_rest_adapter
                    .join_game(&event.game_id)
                    .await;
                if let Err(e) = join_game_result {
                    error!("Error joining game: {}", e);
                }
            }
            GameStatus::STARTED => {
                info!("Game {} Status: Started", event.game_id);
                let robots_you_can_buy_in_first_round: u16 = 5;
                self.game_service_rest_adapter
                    .send_command(Command::create_robot_purchase_command(
                        self.game_service_rest_adapter.get_player_id().unwrap(),
                        robots_you_can_buy_in_first_round,
                    ))
                    .await
                    .expect("Could not send robot purchase command");
            }
            GameStatus::ENDED => {
                info!("Game {} Status: Ended", event.game_id)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::eventinfrastructure::game_event_header::GameEventHeader;
    use crate::rest::game_service_rest_adapter_trait::MockGameServiceRestAdapterTrait;

    use super::*;

    fn get_game_status_event_with_created_game_as_status() -> GameStatusEvent {
        let header = GameEventHeader::default();
        GameStatusEvent {
            game_id: "12345".to_string(),
            gameworld_id: None,
            status: GameStatus::CREATED,
        }
    }

    #[tokio::test]
    async fn test_game_status_event_handler_calls_game_service_rest_adapter_to_join_game_on_created_game_event() {
        let mut game_service_rest_adapter = MockGameServiceRestAdapterTrait::new();
        game_service_rest_adapter
            .expect_join_game()
            .times(1)
            .returning(|_| Ok(true));
        let game_status_event_handler =
            GameStatusEventHandler::new(Arc::new(game_service_rest_adapter));
        game_status_event_handler
            .handle(get_game_status_event_with_created_game_as_status())
            .await;
    }
}
