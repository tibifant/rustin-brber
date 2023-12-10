use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameCreationError {
    #[error("An active game already exists. A game is considered active when its status is either 'CREATED' or 'RUNNING'. Active games have to be closed in order to create a new one.")]
    ActiveGameAlreadyExistsError,
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[derive(Error, Debug)]
pub enum GameServiceError {
    #[error("A request was tried to be sent to the Game Service at but the Game Service was not reachable. {0}")]
    NotReachableError(reqwest::Error),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Player name was taken and while fetching the details of the player the player was not found. This should not happen.")]
    PlayerNotFoundError,
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("
Syntax invalid, Game not ready (has't started yet or already finished) or command invalid. That may happen if the player is trying to send commands for robots that do not belong to him. {0}")]
    MultipleCauseError(String),
    #[error("Player or game not found. {0}")]
    PlayerOrGameNotFound(String),
    #[error("Unexpected error when trying to send a Command: {0}")]
    UnknownError(String),
}
