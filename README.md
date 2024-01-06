# Table of Contents

- [Player Skeleton Rust](#player-skeleton-rust)
- [Requirements](#requirements)
- [Preparation](#preparation)
    - [Configuration](#configuration)
    - [Running the Player](#running-the-player)
    - [Tests](#tests)
    - [Dev Mode](#dev-mode)
- [Event Listening](#event-listening)
- [How to continue from here](#how-to-continue-from-here)
- [Deployments (Near end of project)](#deployments-near-end-of-project)
- [Authors](#authors)

## Player Skeleton Rust

This is the Documentation of the Player Skeleton for the microservice dungeon, which is written in Rust.
You can use this player as basis for your own Rust based Player.
It already implemented the basic functionality of a player:

* Creating(Dev Mode), Registering, Joining, Starting(Dev Mode) and Ending old Games(Dev Mode)
* Listening and Logging for incoming Events
* Parsing basic Events into Event Classes
* Handle Incoming events by calling the Handler of the specific event class (e.g. GameStatusEventHandler)
* Domain Primitives you can use to build your player
* Tests for the basic functionality

## Requirements

Before you start working with the Player Skeleton Rust, ensure that your development environment meets the following
requirements:

- **Rust Programming Language**: The entire codebase is written in Rust, so you need to have Rust installed on your
  machine. You can download it from the [official Rust website](https://www.rust-lang.org/learn/get-started).
- **Cargo**: Cargo is Rust's build system and package manager, typically included with Rust installation. It handles
  Rust project compilation, package management, and dependencies.
- **Local Dev Environment**: Ensure you have access to the Microservice Dungeon environment, as this player is designed
  to interact with it.

Optional requirements for development include:

- **Rust Rover**: IDE for Rust development.

After setting up these requirements, you can proceed with the preparation steps to configure and run the player
skeleton.

## Preparation

To use [this skeleton](https://gitlab.com/the-microservice-dungeon/player-teams/skeletons/player-rust) as the base for
your player development, you need to accomplish the following steps.

First, fork this repository and create a new repository under
the [Player Teams subgroup](https://gitlab.com/the-microservice-dungeon/player-teams) (or in any other Git location).
The fork should be named after your desired player name, for example `player-constantine`.

Now you need to add your player-name to a few files. The required places are marked using TODO comments.
Update the files in `helm-chart/Chart.yaml` `helm-chart/values.yaml` and `.gitlab-ci.yml`.

### Configuration

The player can be configured using the environment variables:

| Environment Variable | Default                    |
|----------------------|----------------------------|
| `GAME_HOST`          | ` http://127.0.0.1 `       |
| `GAME_PORT`          | ` 8080 `                   |
| `PLAYER_NAME`        | ` player-skeleton-rust `   |
| `PLAYER_EMAIL`       | ` rust-skeleton@test.com ` |
| `RABBITMQ_HOST`      | ` 127.0.0.1 `              |
| `RABBITMQ_PORT`      | ` 5672 `                   |
| `RABBITMQ_USERNAME`  | ` admin`                   |
| `RABBITMQ_PASSWORD`  | ` admin`                   |
| `RUST_LOG`           | ` info `                   |
| `DEV_MODE`           | ` false `                  |

### Running the Player

To compile the player, you need to execute the following command:

```shell
cargo build
```

To run the player, you need to execute the following command:

```shell
cargo run
```

### Tests

To run the tests, you just need to execute the following command:

```shell
cargo test
```

### Dev Mode

**Dev mode** is available for local development and can be enabled through an environment variable. It automates the
game creation and start process:

- **To enable**: Set the `DEV_MODE` environment variable to `True`.
- **Note**: This feature is intended **only for local development**.

## Event Listening

The skeleton player utilizes a single messaging queue for all events. It listens asynchronously on the player-owned queue for events,
deserializes them using serde, and dispatches them to the appropriate event handler
via `src.eventinfrastructure.event_dispatcher.rs`. Currently, two event handlers are implemented:

- For `GameStatus` events: `src.game.application.game_status_event_handler.rs`
- For `RoundStatus` events: `src.game.application.round_status_event_handler.py`

## How to continue from here

With the event listening and handling framework in place, the next steps involve:

1. **Set up a Running Local Dev Environmnent**: Set up
   a [Local dev environment](https://gitlab.com/the-microservice-dungeon/devops-team/local-dev-environment) to run &
   test your player in games
   locally.
2. **Understanding Game Events**: Familiarize yourself with the different game events. Consider the information each
   event provides and how it can be used for decision-making in the game.
3. **Add Missing Event Struct**: In case you encounter any missing events you can add them in
   the `src.eventinfrastructure` package. These structs are
   required to deserialize the events received from the Game Service.
4. **Implementing Event Handlers**: Develop your own Event Handlers which implement the **_EventHandler Async Trait_
   ** `src.eventinfrastructure.event_handler.rs` to build your own "view" of the game state. This
   view can be used to make decisions in the game. 
5. **Send Commands** : Once you have a view of the game state you can start to implement sending
   commands `src.domainprimitives.command.command.rs` based on your view of the
   game state. You should probably send commands or your Player does not do anything in the game :D.
6. **Deployments**: Once you are happy with the state of your player it will be time to get the Deployment for it
   running.
7. **Have fun and be creative**: There are many ways to play the game. Be creative and have fun!

## Deployments (Near end of project)

Deployments are probably the last thing you have to do before the project is finished.
Make sure to adjust the `values.yaml` in the `helm-chart` folder to the projects needs!
Speak with the DevOps team if you run into issues.

## Further Reading

- [Learning About Asynchronous Communication](https://the-microservice-dungeon.gitlab.io/docs/docs/reference/asynchronous-communication/)
- [Serde for De/Serialization](https://serde.rs/)
- [Rust Documentation](https://doc.rust-lang.org/)

## Authors

- [Andre Müller](https://gitlab.com/Amueller36)