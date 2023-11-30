mod config;
mod rest;

fn main() {
    env_logger::init();
    let game_service_rest_adapter = rest::game_service_restadapter::GameServiceRESTAdapter::new();
    let test = game_service_rest_adapter.get_open_games().expect("Failed to get open games");
}