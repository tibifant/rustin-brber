use std::error::Error;
use serde::Deserialize;
use super::client::HttpClient;
use super::super::config::CONFIG;

use log::error;
use log::info;
pub struct GameServiceRESTAdapter {
    client : HttpClient
}

impl GameServiceRESTAdapter {
    pub fn new() -> GameServiceRESTAdapter {
        GameServiceRESTAdapter {
            client: HttpClient::new(),
        }
    }

    pub fn get_open_games(&self) -> Result<Vec<GameInfoResponseDto>, Box<dyn Error>> {
        let url = format!("http://{}:{}/games", CONFIG.game_service_host, CONFIG.game_service_port);

        let response = self.client.sync_client.get(&url).send()?;
        let response_text = response.text()?;

        let games: Vec<GameInfoResponseDto> = serde_json::from_str(&response_text)?;

        for game in &games {
            info!("Game: {:?}", game);
        }

        Ok(games)
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameInfoResponseDto {
    pub game_id: String,
    pub game_status : GameStatus,
    pub max_players : u16,
    pub max_rounds : u16,
    pub current_round_number : Option<u16>,
    pub round_length_in_millis : u16,
    pub participating_players : Vec<String>,
}
#[derive(Deserialize, Debug)]
pub enum GameStatus {
    #[serde(rename(deserialize = "created"))]
    CREATED,
    #[serde(rename(deserialize = "started"))]
    STARTED,
    #[serde(rename(deserialize = "finished"))]
    FINISHED
}