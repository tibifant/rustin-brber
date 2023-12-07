use std::error::Error;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::config::CONFIG;
use crate::player::player::Player;
use crate::rest::request::command::command::Command;
use crate::rest::request::create_game_request_body::CreateGameRequestBody;
use crate::rest::request::fetch_player_request_query::FetchPlayerRequestQuery;
use crate::rest::request::register_player_request_body::RegisterPlayerRequestBody;
use crate::rest::response::game_info_response_body::GameInfoResponseBody;

use super::client::HttpClient;
use super::errors::{GameCreationError, PlayerRegistrationError, CommandError};

#[derive(Debug)]
pub struct GameServiceRESTAdapter {
    client: HttpClient,
    game_host: String,
}


impl GameServiceRESTAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpClient::new(),
            game_host: format!("{}:{}", CONFIG.game_service_host, CONFIG.game_service_port),
        }
    }
    pub fn with_game_host(mut self, host: String) -> Self {
        self.game_host = host;
        return self;
    }


    pub async fn get_joinable_games(&self) -> Result<Vec<GameInfoResponseBody>, Box<dyn Error>> {
        let url = format!("{}/games", self.game_host);

        let response = self.client.async_client.get(&url).send().await?;

        let games: Vec<GameInfoResponseBody> = response.json().await?;

        for game in &games {
            info!("Game: {:?}", game);
        }

        Ok(games.iter().filter(|game| game.game_status == GameStatus::CREATED).cloned().collect())
    }


    pub async fn create_game(&self, max_players: u16, rounds: u16) -> Result<GameInfoResponseBody, Box<dyn Error>> {
        let url = format!("{}/games", self.game_host);
        let body = CreateGameRequestBody {
            max_players,
            max_rounds: rounds,
        };

        let response = self.client.async_client.post(&url).json(&body).send().await?;

        match response.status() {
            StatusCode::CREATED => {
                let game: GameInfoResponseBody = response.json().await?;
                Ok(game)
            }
            StatusCode::BAD_REQUEST => {
                Err(Box::new(GameCreationError::ActiveGameAlreadyExistsError))
            }
            _ => {
                Err(Box::new(GameCreationError::UnexpectedError(format!("Unexpected status code: {}", response.status()))))
            }
        }
    }
    pub async fn join_game(&self, game_id: &str) {
        let player_id = self.fetch_player().await.unwrap().player_id;
        let url = format!("{}/games/{}/players/{}", self.game_host, game_id, player_id);

        let response = self.client.async_client.put(&url).send().await;
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        info!("Player joined game: {}", response.text().await.unwrap());
                    }
                    _ => {
                        error!("Player could not join game: {}", response.text().await.unwrap());
                    }
                }
            }
            Err(e) => {
                error!("Player could not join game: {}", e);
            }
        }
    }

    pub async fn send_command(&self, command: Command) -> Result<(), CommandError> {
        let url = format!("{}/commands", self.game_host);
        let response = self.client.async_client.post(&url).json(&command).send().await;
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::CREATED => {
                        info!("Command sent: {}", response.text().await.unwrap());
                        Ok(())
                    }
                    StatusCode::BAD_REQUEST => {
                        Err(CommandError::MultipleCauseError(response.text().await.unwrap()))
                    }
                    StatusCode::NOT_FOUND => {
                        Err(CommandError::PlayerOrGameNotFound(response.text().await.unwrap()))
                    }
                    _ => {
                        Err(CommandError::UnknownError(format!("Unexpected status code: {} \n {}", response.status(), response.text().await.unwrap())))
                    }
                }
            }
            Err(e) => {
                Err(CommandError::UnknownError(e.to_string()))
            }
        }
    }


    pub async fn register_player(&self) -> Result<Player, Box<dyn Error>> {
        let url = format!("{}/players", self.game_host);
        let body = RegisterPlayerRequestBody {
            name: CONFIG.player_name.clone(),
            email: CONFIG.player_email.clone(),
        };
        let response = self.client.async_client.post(&url).json(&body).send().await?;
        let player = match response.status() {
            StatusCode::CREATED => {
                let response_str = response.text().await?;
                serde_json::from_str(&response_str)?
            }
            StatusCode::BAD_REQUEST => {
                info!("Player cannot be registered because it already exists. Fetching player instead.");
                self.fetch_player().await?
            }
            _ => {
                return Err(Box::new(PlayerRegistrationError::UnexpectedError(format!("Unexpected status code: {}", response.status()))));
            }
        };
        Ok(player)
    }


    async fn fetch_player(&self) -> Result<Player, Box<dyn Error>> {
        let url = format!("{}/players", self.game_host);
        let query = FetchPlayerRequestQuery {
            name: CONFIG.player_name.clone(),
            email: CONFIG.player_email.clone(),
        };
        let response = self.client.async_client.get(&url).query(&query).send().await?;
        match response.status() {
            StatusCode::OK => {
                let response_text = response.text().await?;
                let player: Player = serde_json::from_str(&response_text)?;
                Ok(player)
            }
            StatusCode::NOT_FOUND => {
                return Err(Box::new(PlayerRegistrationError::PlayerNotFoundError));
            }
            _ => {
                return Err(Box::new(PlayerRegistrationError::UnexpectedError(format!("Unexpected status code: {}", response.status()))));
            }
        }
    }
}

//TODO: Move in game package when game package is created
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GameStatus {
    CREATED,
    STARTED,
    ENDED,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use uuid::Uuid;

    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    use super::*;


    async fn setup_mock_server_and_client() -> (MockServer, GameServiceRESTAdapter) {
        let mock_server = MockServer::start().await;
        let client = GameServiceRESTAdapter::new().with_game_host(mock_server.uri());
        (mock_server, client)
    }

    #[tokio::test]
    async fn test_create_game_success() {
        let (mock_server, client) = setup_mock_server_and_client().await;

        let fake_response = ResponseTemplate::new(201)
            .set_body_json(GameInfoResponseBody {
                game_id: "1234".to_string(),
                game_status: GameStatus::CREATED,
                max_players: 4,
                max_rounds: 10,
                current_round_number: None,
                round_length_in_millis: 1000,
                participating_players: vec![],
            });

        Mock::given(method("POST"))
            .and(path("/games"))
            .respond_with(fake_response)
            .mount(&mock_server)
            .await;

        let result = client.create_game(4, 10).await;

        let game = result.unwrap();
        assert_eq!(game.game_id, "1234");
        assert_eq!(game.game_status, GameStatus::CREATED);
        assert_eq!(game.max_players, 4);
        assert_eq!(game.max_rounds, 10);
        assert_eq!(game.current_round_number, None);
        assert_eq!(game.round_length_in_millis, 1000);
        assert_eq!(game.participating_players, Vec::<String>::new());
    }

    #[tokio::test]
    async fn test_create_game_active_game_exists() {
        let (mock_server, client) = setup_mock_server_and_client().await;

        Mock::given(method("POST"))
            .and(path("/games"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;

        let result = client.create_game(4, 10).await;
        match result {
            Err(e) => {
                if let Some(specific_error) = e.downcast_ref::<GameCreationError>() {
                    match specific_error {
                        GameCreationError::ActiveGameAlreadyExistsError => assert!(true),
                        _ => assert!(false, "Unexpected error type {:?}", specific_error),
                    }
                } else {
                    assert!(false, "Expected Error of type GameCreationError but was {:?}", e.deref());
                }
            }
            _ => assert!(false, "Expected that GameCreationError is returned but no error was returned."),
        }
    }

    #[tokio::test]
    async fn test_create_game_unexpected_error() {
        let (mock_server, client) = setup_mock_server_and_client().await;

        Mock::given(method("POST"))
            .and(path("/games"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = client.create_game(4, 10).await;

        match result {
            Err(e) => {
                if let Some(expected_error) = e.downcast_ref::<GameCreationError>() {
                    match expected_error {
                        GameCreationError::UnexpectedError(_) => assert!(true),
                        _ => assert!(false, "Expected 'UnexpectedError' but was {:?} ", expected_error)
                    }
                } else {
                    assert!(false, "Expected Error Type DungeonPlayerError but was {:?}", e)
                }
            }
            _ => assert!(false)
        }
    }

    #[tokio::test]
    async fn test_register_player_success() {
        let (mock_server, client) = setup_mock_server_and_client().await;
        let id = Uuid::new_v4();

        let fake_response = ResponseTemplate::new(201)
            .set_body_json(Player {
                player_id: id,
                name: "test".to_string(),
                email: "test@mail.de".to_string(),
                player_exchange: "player-test".to_string(),
                player_queue: "player-test".to_string(),
            });

        Mock::given(method("POST"))
            .and(path("/players"))
            .respond_with(fake_response)
            .mount(&mock_server)
            .await;


        let result = client.register_player().await;

        let player = result.unwrap();
        assert_eq!(player.player_id, id);
        assert_eq!(player.name, "test");
        assert_eq!(player.email, "test@mail.de");
        assert_eq!(player.player_exchange, "player-test");
        assert_eq!(player.player_queue, "player-test");
    }

    #[tokio::test]
    async fn test_register_player_already_exists_but_returns_fetched_player() {
        let (mock_server, client) = setup_mock_server_and_client().await;
        let id = Uuid::new_v4();

        let fake_response = ResponseTemplate::new(400)
            .set_body_json(vec![
                "message", "Player already exists.",
            ]);

        Mock::given(method("POST"))
            .and(path("/players"))
            .respond_with(fake_response)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/players"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(Player {
                    player_id: id,
                    name: "test".to_string(),
                    email: "test@mail.de".to_string(),
                    player_exchange: "player-test".to_string(),
                    player_queue: "player-test".to_string(),
                }))
            .mount(&mock_server)
            .await;


        let result = client.register_player().await;

        match result {
            Err(e) => {
                assert!(false, "Expected that player is fetched but error was returned: {:?}", e)
            }
            _ => assert!(true),
        }
    }
}
