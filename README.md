# Rust Player Skeleton for the Microservice Dungeon

## Configuration
The player can be configured using the environment variables:

| Environment Variable | Default                    |
|----------------------|----------------------------|
| `GAME_SERVICE_HOST`  | ` http://127.0.0.1 `       |
| `GAME_SERVICE_PORT`  | ` 8080 `                   |
| `PLAYER_NAME`        | ` player-skeleton-rust `   |
| `PLAYER_EMAIL`       | ` rust-skeleton@test.com ` |
| `RABBITMQ_HOST`      | ` 127.0.0.1 `              |
| `RABBITMQ_PORT`      | ` 5672 `                   |
| `RABBITMQ_USERNAME`  | ` admin`                   |
| `RABBITMQ_PASSWORD`  | ` admin`                   |
| `RUST_LOG`           | ` Info `                   |
| `DEV_MODE`           | ` true `                   |

## Dev Mode
The player can be run in dev mode by setting the `DEV_MODE` environment variable to `true`. 
This will automatically shut down all existing games on player startup

## Authors
Andre Müller [Amueller36]