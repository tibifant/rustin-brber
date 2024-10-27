use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{domainprimitives::{location::mineable_resource_type::MineableResourceType, purchasing::trade_item_type::TradeItemType}, eventinfrastructure::{event_handler::EventHandler, trading::{bank_account_initialized_event::BankAccountInitializedEvent, bank_account_transaction_booked::BankAccountTransactionBookedEvent, tradable_prices_event::TradablePricesEvent}}, game::application::game_logic_service::GameLogicService};


pub struct BankAccountInitializedEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl BankAccountInitializedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<BankAccountInitializedEvent> for BankAccountInitializedEventHandler {
  async fn handle(&self, event: BankAccountInitializedEvent) {
    let mut game_mut = self.game.lock().await;
    if event.player_id == game_mut.game_data.player_id {
      game_mut.balance_update(event.balance);
    }
  }
}

pub struct BankAccountTransactionBookedEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl BankAccountTransactionBookedEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<BankAccountTransactionBookedEvent> for BankAccountTransactionBookedEventHandler {
  async fn handle(&self, event: BankAccountTransactionBookedEvent) {
    let mut game_mut = self.game.lock().await;
    if event.player_id == game_mut.game_data.player_id {
      game_mut.balance_update(event.balance);
    }
  }
}

pub struct TradablePricesEventHandler {
  game: Arc<Mutex<GameLogicService>>,
}

impl TradablePricesEventHandler {
  pub fn new(game: Arc<Mutex<GameLogicService>>) -> Self {
    Self {
      game,
    }
  }
}

#[async_trait]
impl EventHandler<TradablePricesEvent> for TradablePricesEventHandler {
  async fn handle(&self, event: TradablePricesEvent) {
    let mut game_mut = self.game.lock().await;
    for item in event.items {
      match item.name.as_str() {
        "MINING_SPEED_1" => game_mut.update_item_price(TradeItemType::MiningSpeed1, item.price as f32),
        "MINING_SPEED_2" => game_mut.update_item_price(TradeItemType::MiningSpeed2, item.price as f32),
        "MINING_SPEED_3" => game_mut.update_item_price(TradeItemType::MiningSpeed3, item.price as f32),
        "MINING_SPEED_4" => game_mut.update_item_price(TradeItemType::MiningSpeed4, item.price as f32),
        "MINING_SPEED_5" => game_mut.update_item_price(TradeItemType::MiningSpeed5, item.price as f32),
        "MAX_ENERGY_1" => game_mut.update_item_price(TradeItemType::MaxEnergy1, item.price as f32),
        "MAX_ENERGY_2" => game_mut.update_item_price(TradeItemType::MaxEnergy2, item.price as f32),
        "MAX_ENERGY_3" => game_mut.update_item_price(TradeItemType::MaxEnergy3, item.price as f32),
        "MAX_ENERGY_4" => game_mut.update_item_price(TradeItemType::MaxEnergy4, item.price as f32),
        "MAX_ENERGY_5" => game_mut.update_item_price(TradeItemType::MaxEnergy5, item.price as f32),
        "ENERGY_REGEN_1" => game_mut.update_item_price(TradeItemType::EnergyRegen1, item.price as f32),
        "ENERGY_REGEN_2" => game_mut.update_item_price(TradeItemType::EnergyRegen2, item.price as f32),
        "ENERGY_REGEN_3" => game_mut.update_item_price(TradeItemType::EnergyRegen3, item.price as f32),
        "ENERGY_REGEN_4" => game_mut.update_item_price(TradeItemType::EnergyRegen4, item.price as f32),
        "ENERGY_REGEN_5" => game_mut.update_item_price(TradeItemType::EnergyRegen5, item.price as f32),
        "STORAGE_1" => game_mut.update_item_price(TradeItemType::Storage1, item.price as f32),
        "STORAGE_2" => game_mut.update_item_price(TradeItemType::Storage2, item.price as f32),
        "STORAGE_3" => game_mut.update_item_price(TradeItemType::Storage3, item.price as f32),
        "STORAGE_4" => game_mut.update_item_price(TradeItemType::Storage4, item.price as f32),
        "STORAGE_5" => game_mut.update_item_price(TradeItemType::Storage5, item.price as f32),
        "MINING_1" => game_mut.update_item_price(TradeItemType::Mining1, item.price as f32),
        "MINING_2" => game_mut.update_item_price(TradeItemType::Mining2, item.price as f32),
        "MINING_3" => game_mut.update_item_price(TradeItemType::Mining3, item.price as f32),
        "MINING_4" => game_mut.update_item_price(TradeItemType::Mining4, item.price as f32),
        "MINING_5" => game_mut.update_item_price(TradeItemType::Mining5, item.price as f32),
        "HEALTH_1" => game_mut.update_item_price(TradeItemType::Health1, item.price as f32),
        "HEALTH_2" => game_mut.update_item_price(TradeItemType::Health2, item.price as f32),
        "HEALTH_3" => game_mut.update_item_price(TradeItemType::Health3, item.price as f32),
        "HEALTH_4" => game_mut.update_item_price(TradeItemType::Health4, item.price as f32),
        "HEALTH_5" => game_mut.update_item_price(TradeItemType::Health5, item.price as f32),
        "DAMAGE_1" => game_mut.update_item_price(TradeItemType::Damage1, item.price as f32),
        "DAMAGE_2" => game_mut.update_item_price(TradeItemType::Damage2, item.price as f32),
        "DAMAGE_3" => game_mut.update_item_price(TradeItemType::Damage3, item.price as f32),
        "DAMAGE_4" => game_mut.update_item_price(TradeItemType::Damage4, item.price as f32),
        "DAMAGE_5" => game_mut.update_item_price(TradeItemType::Damage5, item.price as f32),
        "ENERGY_RESTORE" => game_mut.update_item_price(TradeItemType::EnergyRestore, item.price as f32),
        "HEALTH_RESTORE" => game_mut.update_item_price(TradeItemType::HealthRestore, item.price as f32),
        "ROBOT" => game_mut.update_item_price(TradeItemType::Robot, item.price as f32),
        "COAL" => game_mut.update_resource_price(MineableResourceType::COAL, item.price as f32),
        "IRON" => game_mut.update_resource_price(MineableResourceType::IRON, item.price as f32),
        "GEM" => game_mut.update_resource_price(MineableResourceType::GEM, item.price as f32),
        "GOLD" => game_mut.update_resource_price(MineableResourceType::GOLD, item.price as f32),
        "PLATIN" => game_mut.update_resource_price(MineableResourceType::PLATIN, item.price as f32),
        _ => {},
      }
    }
  }
}