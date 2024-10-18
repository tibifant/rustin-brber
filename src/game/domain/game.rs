use tracing::error;

use crate::game::domain::game_status::GameStatus;
use crate::repository::Identifiable;
use crate::rest::response::game_info_response_body::GameInfoResponseBody;

#[derive(Clone, Debug)]
pub struct Game {
    pub game_id: String,
    pub game_status: GameStatus,
    pub current_round_number: u16,
    pub our_player_has_joined: bool,
    pub max_rounds: u16,
}

impl From<&GameInfoResponseBody> for Game {
    fn from(value: &GameInfoResponseBody) -> Self {
        Self {
            game_id: value.game_id.clone(),
            game_status: value.game_status.clone(),
            current_round_number: value.current_round_number.unwrap_or(0),
            our_player_has_joined: false,
            max_rounds: value.max_rounds,
        }
    }
}

impl Game {
    pub fn newly_created_game(game_id: String) -> Self {
        Self {
            game_id,
            game_status: GameStatus::CREATED,
            current_round_number: 0,
            our_player_has_joined: false,
            max_rounds: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.game_status = GameStatus::STARTED;
    }

    pub fn end_game(&mut self) {
        self.game_status = GameStatus::ENDED;
    }

    pub fn is_started(&self) -> bool {
        self.game_status == GameStatus::STARTED
    }

    pub fn is_ended(&self) -> bool {
        self.game_status == GameStatus::ENDED
    }

    pub fn check_if_our_player_has_joined(
        &mut self,
        names_of_joined_players: &[String],
        player_name: &str,
    ) -> bool {
        if names_of_joined_players
            .iter()
            .any(|name| name == player_name)
        {
            self.our_player_has_joined = true;
        }
        self.our_player_has_joined
    }

    pub fn start_round(&mut self) {
        if self.current_round_number > self.max_rounds {
            error!(
                "Can't increment Round Number, game {} has already hit max Rounds",
                self.game_id
            );
            return;
        }
        self.current_round_number += 1;

        // TODO execute commands and stuff
    }
}

impl Identifiable for Game {
    fn id(&self) -> String {
        self.game_id.clone()
    }
}
