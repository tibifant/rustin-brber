use std::env;
use lazy_static::lazy_static;
pub struct Config {
    pub game_service_host: String,
    pub game_service_port: u16,
    pub player_name: String,
    pub player_email: String,
    pub rabbitmq_host: String,
    pub rabbitmq_port: u16,
    pub rabbitmq_username: String,
    pub rabbitmq_password: String,
}

impl Config {
    fn new() -> Self {
        Self {
            game_service_host: env::var("GAME_SERVICE_HOST").unwrap_or("http://127.0.0.1".to_string()),
            game_service_port: env::var("GAME_SERVICE_PORT").unwrap_or(8080.to_string()).parse::<u16>().unwrap(),
            player_name: env::var("PLAYER_NAME").unwrap_or("player-skeleton-rust".to_string()),
            player_email: env::var("PLAYER_EMAIL").unwrap_or("rust-skeleton@test.com".to_string()),
            rabbitmq_host: env::var("RABBITMQ_HOST").unwrap_or("127.0.0.1".to_string()),
            rabbitmq_port: env::var("RABBITMQ_PORT").unwrap_or(5672.to_string()).parse::<u16>().unwrap(),
            rabbitmq_username: env::var("RABBITMQ_USERNAME").unwrap_or("admin".to_string()),
            rabbitmq_password: env::var("RABBITMQ_PASSWORD").unwrap_or("admin".to_string()),
        }
    }
}

lazy_static! {
    pub static ref CONFIG : Config = Config::new();
}