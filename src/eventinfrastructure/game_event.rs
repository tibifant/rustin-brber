use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::game_event_header::GameEventHeader;
use crate::eventinfrastructure::game_event_type::GameEventType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEvent {
   pub header: GameEventHeader,
   pub event: GameEventType,
}

