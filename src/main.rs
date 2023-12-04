mod config;
mod rest;
mod dungeon_player_error;

mod player;
mod dungeon_player_startup_handler;
mod rabbitmq;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut startup_handler = dungeon_player_startup_handler::DungeonPlayerStartupHandler::new();
    startup_handler.register_player().await;
}