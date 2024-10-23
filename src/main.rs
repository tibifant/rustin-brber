mod config;
mod domainprimitives;
mod dungeon_player_startup_handler;
mod eventinfrastructure;
mod game;
mod player;
mod repository;
mod rest;
mod robot;
mod game_logic;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut startup_handler =
        dungeon_player_startup_handler::DungeonPlayerStartupHandler::new().await;
    startup_handler.start().await;
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for CTRL-C");
}
