use serde::Deserialize;

use crate::eventinfrastructure::game::game_status_event::GameStatusEvent;
use crate::eventinfrastructure::game::round_status_event::RoundStatusEvent;
use crate::eventinfrastructure::game_error_event::GameErrorEvent;
use crate::eventinfrastructure::map::planet_discovered_event::PlanetDiscoveredEvent;
use crate::eventinfrastructure::map::planet_resource_mined_event::PlanetResourceMinedEvent;
use crate::eventinfrastructure::robot::robot_attacked_event::RobotAttackedEvent;
use crate::eventinfrastructure::robot::robot_moved_event::RobotMovedEvent;
use crate::eventinfrastructure::robot::robot_regenerated_event::RobotRegeneratedEvent;
use crate::eventinfrastructure::robot::robot_resource_mined_event::RobotResourceMinedEvent;
use crate::eventinfrastructure::robot::robot_resource_removed_event::RobotResourceRemovedEvent;
use crate::eventinfrastructure::robot::robot_restored_attributes_event::RobotRestoredAttributesEvent;
use crate::eventinfrastructure::robot::robot_spawned_event::RobotSpawnedEvent;
use crate::eventinfrastructure::robot::robot_upgraded_event::RobotUpgradedEvent;
use crate::eventinfrastructure::robot::robots_revealed_event::RobotsRevealedEvent;
use crate::eventinfrastructure::trading::bank_account_cleared_event::BankAccountClearedEvent;
use crate::eventinfrastructure::trading::bank_account_initialized_event::BankAccountInitializedEvent;
use crate::eventinfrastructure::trading::bank_account_transaction_booked::BankAccountTransactionBookedEvent;
use crate::eventinfrastructure::trading::tradable_bought_event::TradableBoughtEvent;
use crate::eventinfrastructure::trading::tradable_prices_event::TradablePricesEvent;
use crate::eventinfrastructure::trading::tradable_sold_event::TradableSoldEvent;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "event")]
pub enum GameEventBodyType {
    //Status Events
    GameStatus(GameStatusEvent),
    RoundStatus(RoundStatusEvent),

    //Trading/Player Events
    TradablePrices(TradablePricesEvent),
    BankAccountInitialized(BankAccountInitializedEvent),
    BankAccountCleared(BankAccountClearedEvent),
    BankAccountTransactionBooked(BankAccountTransactionBookedEvent),
    TradableBought(TradableBoughtEvent),
    TradableSold(TradableSoldEvent),

    //Robot Events
    RobotSpawned(RobotSpawnedEvent),
    RobotAttacked(RobotAttackedEvent),
    RobotMoved(RobotMovedEvent),
    RobotRegenerated(RobotRegeneratedEvent),
    RobotUpgraded(RobotUpgradedEvent),
    RobotResourceMined(RobotResourceMinedEvent),
    RobotResourceRemoved(RobotResourceRemovedEvent),
    RobotRestoredAttributes(RobotRestoredAttributesEvent),
    RobotsRevealed(RobotsRevealedEvent),

    //Map
    PlanetDiscovered(PlanetDiscoveredEvent),
    ResourceMined(PlanetResourceMinedEvent),
    #[serde(alias = "error")]
    ErrorEvent(GameErrorEvent),
}
