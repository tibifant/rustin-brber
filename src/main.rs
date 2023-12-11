mod config;
mod rest;
mod dungeon_player_error;
mod player;
mod dungeon_player_startup_handler;
mod eventinfrastructure;

mod domainprimitives;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut startup_handler = dungeon_player_startup_handler::DungeonPlayerStartupHandler::new().await;
    startup_handler.start().await;
    tokio::signal::ctrl_c().await.expect("Failed to listen for CTRL-C");
}