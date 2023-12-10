use serde::Deserialize;

use crate::eventinfrastructure::game_event_header::GameEventHeader;
use crate::eventinfrastructure::game_event_body_type::GameEventBodyType;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameEvent {
   pub header: GameEventHeader,
   pub event_body: GameEventBodyType,
}

