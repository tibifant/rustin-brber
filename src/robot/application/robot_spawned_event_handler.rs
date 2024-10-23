use std::sync::Arc;

use async_trait::async_trait;

use crate::eventinfrastructure::event_handler::EventHandler;
use crate::eventinfrastructure::robot::robot_spawned_event::RobotSpawnedEvent;
use crate::robot::application::robot_application_service::RobotApplicationService;
use crate::robot::domain::robot::{Inventory, MinimalRobot, Robot};

pub struct RobotSpawnedEventHandler {
    robot_application_service: Arc<RobotApplicationService>,
}

impl RobotSpawnedEventHandler {
    pub fn new(robot_application_service: Arc<RobotApplicationService>) -> Self {
        Self {
            robot_application_service
        }
    }
}

#[async_trait]
impl EventHandler<RobotSpawnedEvent> for RobotSpawnedEventHandler {
    async fn handle(&self, event: RobotSpawnedEvent) {
        let r = event.robot;
        let robot_info = MinimalRobot::new(r.robot_id.to_string(), r.planet.planet_id.to_string(), r.robot_attributes.energy, r.robot_attributes.health, r.robot_levels.health_level, r.robot_levels.damage_level, r.robot_levels.mining_speed_level, r.robot_levels.mining_level, r.robot_levels.energy_level, r.robot_levels.energy_regen_level, r.inventory.storage_level);
        let r_i = r.inventory.resources;
        let inventory = Inventory::new(r_i.coal, r_i.iron, r_i.gold, r_i.gem, r_i.platin, r.inventory.full);
        let robot = Robot::new(robot_info, inventory, r.robot_attributes.max_health, r.robot_attributes.max_energy, r.robot_attributes.energy_regen, r.robot_attributes.attack_damage, r.robot_attributes.mining_speed);
        self.robot_application_service.add_robot(r.player_id.as_str(), robot).await;
    }
}
