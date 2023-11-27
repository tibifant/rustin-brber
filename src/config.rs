use std::env;
use lazy_static::lazy_static;
struct Config {
    game_service_host: String,
    game_service_port: u16,
    player_name: String,
    player_email: String
}

impl Config {
    fn new() -> Self {
        Self {
            game_service_host: env::var("GAME_SERVICE_HOST").unwrap_or("0.0.0.0".to_string()),
            game_service_port: env::var("GAME_SERVICE_PORT").unwrap_or(8080.to_string()).parse::<u16>().unwrap(),
            player_name: env::var("PLAYER_NAME").unwrap_or("player-skeleton-rust".to_string()),
            player_email: env::var("PLAYER_EMAIL").unwrap_or("rust-skeleton@test.com".to_string()),
        }
    }
}

lazy_static! {
    static ref CONFIG : Config = Config::new().expect("Error Loading the Config, are all environment Variables set?");
}