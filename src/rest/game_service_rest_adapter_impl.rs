use std::error::Error;
use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::StatusCode;
use tracing::{error, info};

use crate::config::CONFIG;
use crate::domainprimitives::command::command::Command;
use crate::game::domain::game_status::GameStatus;
use crate::player::domain::player::Player;
use crate::rest::game_service_rest_adapter_trait::GameServiceRestAdapterTrait;
use crate::rest::request::create_game_request_body::CreateGameRequestBody;
use crate::rest::request::fetch_player_request_query::FetchPlayerRequestQuery;
use crate::rest::request::patch_round_duration_request_body::PatchRoundDurationRequestBody;
use crate::rest::request::register_player_request_body::RegisterPlayerRequestBody;
use crate::rest::response::command_info_response::CommandInfoResponse;
use crate::rest::response::created_game_info_response_body::CreatedGameInfoResponseBody;
use crate::rest::response::game_info_response_body::GameInfoResponseBody;

use super::client::HttpClient;
use super::errors::{CommandError, GameCreationError, GameServiceError, PlayerError};

#[derive(Debug)]
pub struct GameServiceRestAdapterImpl {
    client: HttpClient,
    game_host: String,
}

impl GameServiceRestAdapterImpl {
    pub fn new() -> Self {
        Self {
            client: HttpClient::new(),
            game_host: format!("{}:{}", CONFIG.game_host, CONFIG.game_port),
        }
    }
    pub fn with_game_host(mut self, host: String) -> Self {
        self.game_host = host;
        return self;
    }

    fn handle_reqwest_error(e: reqwest::Error) -> Box<dyn Error> {
        if e.is_connect() {
            Box::new(GameServiceError::NotReachableError(e))
        } else {
            Box::new(GameServiceError::UnexpectedError(e.to_string()))
        }
    }
}

#[async_trait]
impl GameServiceRestAdapterTrait for GameServiceRestAdapterImpl {
    async fn get_player_id(&self) -> Option<String> {
        let player = self.fetch_player().await;
        if let Ok(player) = player {
            return player.player_id;
        }
        return None;
    }
    async fn get_all_games(&self) -> Result<Vec<GameInfoResponseBody>, Box<dyn Error>> {
        let url = format!("{}/games", self.game_host);
        let response = self
            .client
            .async_client
            .get(&url)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;

        if response.status() != StatusCode::OK {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "".to_string());
            return Err(Box::new(GameServiceError::UnexpectedError(format!(
                "Unexpected status code: {} \n Error Message: {}",
                status, text
            ))));
        }

        let games: Vec<GameInfoResponseBody> = response.json().await?;

        for game in &games {
            info!("Game: {:?}", game);
        }

        Ok(games)
    }

    async fn create_game(
        &self,
        max_players: u16,
        rounds: u16,
    ) -> Result<CreatedGameInfoResponseBody, Box<dyn Error>> {
        let url = format!("{}/games", self.game_host);
        let body = CreateGameRequestBody {
            max_players,
            max_rounds: rounds,
        };

        let response = self
            .client
            .async_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;

        match response.status() {
            StatusCode::CREATED => {
                let game: CreatedGameInfoResponseBody = response.json().await?;
                Ok(game)
            }
            StatusCode::BAD_REQUEST => {
                Err(Box::new(GameCreationError::ActiveGameAlreadyExistsError))
            }
            _ => Err(Box::new(GameCreationError::UnexpectedError(format!(
                "Unexpected status code: {}",
                response.status()
            )))),
        }
    }
    async fn join_game(&self, game_id: &str) -> Result<bool, Box<dyn Error>> {
        let player_id = self.fetch_player().await.unwrap().player_id.unwrap();
        let url = format!("{}/games/{}/players/{}", self.game_host, game_id, player_id);

        let response = self
            .client
            .async_client
            .put(&url)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;
        return match response.status() {
            StatusCode::OK => Ok(true),
            StatusCode::BAD_REQUEST => {
                error!("Failed to join game. Its either full or has already started.");
                Ok(false)
            }
            StatusCode::NOT_FOUND => {
                error!("Player or game not found.");
                Ok(false)
            }
            _ => {
                error!("Unknown error occured when trying to join a game!");
                Ok(false)
            }
        };
    }

    async fn send_command(&self, command: Command) -> Result<CommandInfoResponse, Box<dyn Error>> {
        let url = format!("{}/commands", self.game_host);
        let response = self
            .client
            .async_client
            .post(&url)
            .json(&command)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;
        match response.status() {
            StatusCode::CREATED => {
                let command_info_response = response.json::<CommandInfoResponse>().await?;
                info!("Command sent: {:?}", command_info_response);
                Ok(command_info_response)
            }
            StatusCode::BAD_REQUEST => Err(Box::new(CommandError::MultipleCauseError(
                response.text().await.unwrap(),
            ))),
            StatusCode::NOT_FOUND => Err(Box::new(CommandError::PlayerOrGameNotFound(
                response.text().await.unwrap(),
            ))),
            code => Err(Box::new(CommandError::UnknownError(format!(
                "Unexpected status code: {} \n {}",
                code,
                response.text().await.unwrap()
            )))),
        }
    }

    async fn register_player(&self) -> Result<Player, Box<dyn Error>> {
        let url = format!("{}/players", self.game_host);
        let body = RegisterPlayerRequestBody {
            name: CONFIG.player_name.clone(),
            email: CONFIG.player_email.clone(),
        };
        let response = self
            .client
            .async_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;

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
                return Err(Box::new(PlayerError::UnexpectedError(format!(
                    "Unexpected status code: {}",
                    response.status()
                ))));
            }
        };
        Ok(player)
    }

    async fn patch_round_duration(
        &self,
        game_id: &str,
        round_duration_in_millis: u64,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/games/{}/duration", self.game_host, game_id);
        let body = PatchRoundDurationRequestBody {
            duration: round_duration_in_millis,
        };
        let response = self
            .client
            .async_client
            .patch(&url)
            .json(&body)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;
        match response.status() {
            StatusCode::OK => {
                info!(
                    "Round duration patched successfully to {}ms for {}!",
                    round_duration_in_millis, game_id
                );
            }
            StatusCode::BAD_REQUEST => {
                error!("Failed to patch round duration. Round duration must be greater than 0.");
            }
            StatusCode::NOT_FOUND => {
                error!(
                    "Failed to patch round duration. Game {} could not be found.",
                    game_id
                );
            }
            _ => {
                error!("Unknown error occured when trying to patch round duration!");
            }
        }
        Ok(())
    }

    async fn fetch_player(&self) -> Result<Player, Box<dyn Error>> {
        let url = format!("{}/players", self.game_host);
        let query = FetchPlayerRequestQuery {
            name: CONFIG.player_name.clone(),
            email: CONFIG.player_email.clone(),
        };
        let response = self
            .client
            .async_client
            .get(&url)
            .query(&query)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;
        match response.status() {
            StatusCode::OK => {
                let response_text = response.text().await?;
                let player: Player = serde_json::from_str(&response_text)?;
                Ok(player)
            }
            StatusCode::NOT_FOUND => {
                return Err(Box::new(PlayerError::PlayerNotFoundError));
            }
            _ => {
                return Err(Box::new(PlayerError::UnexpectedError(format!(
                    "Unexpected status code: {}",
                    response.status()
                ))));
            }
        }
    }

    async fn start_game(&self, game_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/games/{}/gameCommands/start", self.game_host, game_id);
        let response = self
            .client
            .async_client
            .post(&url)
            .send()
            .await
            .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;
        match response.status() {
            StatusCode::CREATED => {
                info!(
                    "Started Game {} successfully! {:?}",
                    game_id,
                    response.text().await.unwrap()
                );
            }
            StatusCode::BAD_REQUEST => {
                error!("Game {} is in a state that prevents it from being started. Its either running or closed. {:?}",game_id, response.text().await.unwrap());
            }
            StatusCode::NOT_FOUND => {
                error!(
                    "Game {} could not be found: {:?}",
                    game_id,
                    response.text().await.unwrap()
                );
            }
            _ => {
                error!(
                    "Unknown error occured when trying to start a game! {:?}",
                    response.text().await.unwrap()
                );
            }
        }
        Ok(())
    }

    async fn end_all_existing_games(&self) -> Result<(), Box<dyn Error>> {
        let games = self
            .get_all_games()
            .await
            .map_err(|e| error!("Couldn't end existing games {}", e))
            .unwrap();
        for game in games {
            let url = format!("{}/games/{}/gameCommands/end", self.game_host, game.game_id);
            if game.game_status == GameStatus::CREATED {
                //start game before ending
                self.start_game(&game.game_id)
                    .await
                    .map_err(|e| error!("Couldn't start game {}", e))
                    .unwrap();
                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            }
            let response = self
                .client
                .async_client
                .post(&url)
                .send()
                .await
                .map_err(GameServiceRestAdapterImpl::handle_reqwest_error)?;

            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

            match response.status() {
                StatusCode::CREATED => {
                    info!("Ended game {:?} successfully!", game.game_id);
                }
                StatusCode::BAD_REQUEST => {
                    error!("Failed to end game {:?}. Game is in a state ({:?}) that prevents it from beeing stopped.", game.game_id, game.game_status);
                }
                StatusCode::NOT_FOUND => {
                    error!(
                        "Game {} can't be ended because it could not be found: {:?}",
                        game.game_id, game
                    );
                }
                _ => {
                    error!("Game could not be ended: {:?}", game);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    async fn setup_mock_server_and_client() -> (MockServer, GameServiceRestAdapterImpl) {
        let mock_server = MockServer::start().await;
        let client = GameServiceRestAdapterImpl::new().with_game_host(mock_server.uri());
        (mock_server, client)
    }

    #[tokio::test]
    async fn test_create_game_success() {
        let (mock_server, client) = setup_mock_server_and_client().await;

        let fake_response = ResponseTemplate::new(201).set_body_json(CreatedGameInfoResponseBody {
            game_id: "5678".to_string(),
        });

        Mock::given(method("POST"))
            .and(path("/games"))
            .respond_with(fake_response)
            .mount(&mock_server)
            .await;

        let result = client.create_game(4, 10).await;

        let game = result.unwrap();
        assert_eq!(game.game_id, "5678");
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
                    assert!(
                        false,
                        "Expected Error of type GameCreationError but was {:?}",
                        e.deref()
                    );
                }
            }
            _ => assert!(
                false,
                "Expected that GameCreationError is returned but no error was returned."
            ),
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
                        _ => assert!(
                            false,
                            "Expected 'UnexpectedError' but was {:?} ",
                            expected_error
                        ),
                    }
                } else {
                    assert!(
                        false,
                        "Expected Error Type DungeonPlayerError but was {:?}",
                        e
                    )
                }
            }
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_register_player_success() {
        let (mock_server, client) = setup_mock_server_and_client().await;
        let id = "1234".to_string();

        let fake_response = ResponseTemplate::new(201).set_body_json(Player {
            player_id: Some(id.clone()),
            game_id: None,
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
        assert_eq!(player.player_id.unwrap(), id);
        assert_eq!(player.name, "test");
        assert_eq!(player.email, "test@mail.de");
        assert_eq!(player.player_exchange, "player-test");
        assert_eq!(player.player_queue, "player-test");
    }

    #[tokio::test]
    async fn test_register_player_already_exists_but_returns_fetched_player() {
        let (mock_server, client) = setup_mock_server_and_client().await;
        let id = "1234".to_string();

        let fake_response =
            ResponseTemplate::new(400).set_body_json(vec!["message", "Player already exists."]);

        Mock::given(method("POST"))
            .and(path("/players"))
            .respond_with(fake_response)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/players"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Player {
                player_id: id.into(),
                game_id: None,
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
                assert!(
                    false,
                    "Expected that player is fetched but error was returned: {:?}",
                    e
                )
            }
            _ => assert!(true),
        }
    }
}
