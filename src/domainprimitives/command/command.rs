use serde::{Deserialize, Serialize};

use crate::domainprimitives::command::command_object::CommandObject;
use crate::domainprimitives::command::command_type::CommandType;
use crate::domainprimitives::purchasing::robot_restoration_type::RobotRestorationType;
use crate::domainprimitives::purchasing::robot_upgrade::RobotUpgrade;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub player_id: String,
    #[serde(rename = "type")]
    command_type: CommandType,
    #[serde(rename = "data")]
    command_object: CommandObject,
}

impl Command {
    pub fn as_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn create_movement_command(
        player_id: String,
        robot_id: String,
        planet_id: String,
    ) -> Command {
        Command {
            player_id,
            command_type: CommandType::MOVEMENT,
            command_object: CommandObject::new()
                .with_robot_id(robot_id)
                .with_planet_id(planet_id),
        }
    }

    pub fn create_item_purchase_command(
        player_id: String,
        item_name: String,
        item_quantity: u16,
    ) -> Command {
        Command {
            player_id,
            command_type: CommandType::BUYING,
            command_object: CommandObject::new()
                .with_item_name(item_name)
                .with_item_quantity(item_quantity),
        }
    }

    pub fn create_robot_purchase_command(player_id: String, amount: u16) -> Command {
        let robot_item_name = String::from("ROBOT");
        Command {
            player_id,
            command_type: CommandType::BUYING,
            command_object: CommandObject::new()
                .with_item_name(robot_item_name)
                .with_item_quantity(amount),
        }
    }

    pub fn create_robot_upgrade_command(
        player_id: String,
        robot_id: String,
        upgrade_name: &RobotUpgrade,
    ) -> Command {
        Command {
            player_id,
            command_type: CommandType::BUYING,
            command_object: CommandObject::new()
                .with_robot_id(robot_id)
                .with_item_name(upgrade_name.to_string_for_command())
                .with_item_quantity(1),
        }
    }

    pub fn create_robot_mine_command(
        player_id: String,
        robot_id: String,
        planet_id: String,
    ) -> Command {
        Command {
            player_id,
            command_type: CommandType::MINING,
            command_object: CommandObject::new()
                .with_robot_id(robot_id)
                .with_planet_id(planet_id),
        }
    }

    pub fn create_robot_sell_inventory_command(player_id: String, robot_id: String) -> Command {
        Command {
            player_id,
            command_type: CommandType::SELLING,
            command_object: CommandObject::new().with_robot_id(robot_id),
        }
    }

    pub fn create_robot_regenerate_command(player_id: String, robot_id: String) -> Command {
        Command {
            player_id,
            command_type: CommandType::REGENERATE,
            command_object: CommandObject::new().with_robot_id(robot_id),
        }
    }

    pub fn create_robot_purchase_energy_restore_command(
        player_id: String,
        robot_id: String,
    ) -> Command {
        let energy_restore_item_name =
            serde_json::to_string(&RobotRestorationType::EnergyRestore).unwrap();
        Command {
            player_id,
            command_type: CommandType::BUYING,
            command_object: CommandObject::new()
                .with_robot_id(robot_id)
                .with_item_name(energy_restore_item_name)
                .with_item_quantity(1),
        }
    }

    pub fn create_robot_purchase_health_restore_command(
        player_id: String,
        robot_id: String,
    ) -> Command {
        let health_restore_item_name =
            serde_json::to_string(&RobotRestorationType::HealthRestore).unwrap();
        Command {
            player_id,
            command_type: CommandType::BUYING,
            command_object: CommandObject::new()
                .with_robot_id(robot_id)
                .with_item_name(health_restore_item_name)
                .with_item_quantity(1),
        }
    }

    pub fn create_robot_attack_command(
        player_id: String,
        robot_id: String,
        target_id: String,
    ) -> Command {
        Command {
            player_id,
            command_type: CommandType::BATTLE,
            command_object: CommandObject::new()
                .with_robot_id(robot_id)
                .with_target_id(target_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domainprimitives::purchasing::robot_upgrade_type::RobotUpgradeType;

    use super::*;

    #[test]
    fn test_create_movement_command() {
        let player_id = String::from("player_id");
        let robot_id = String::from("robot_id");
        let planet_id = String::from("planet_id");
        let command = Command::create_movement_command(
            player_id.clone(),
            robot_id.clone(),
            planet_id.clone(),
        );
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::MOVEMENT);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
        assert_eq!(command.command_object.planet_id, Some(planet_id));
    }

    #[test]
    fn test_create_item_purchase_command() {
        let player_id = String::from("player_id");
        let item_name = String::from("item_name");
        let item_quantity = 10;
        let command = Command::create_item_purchase_command(
            player_id.clone(),
            item_name.clone(),
            item_quantity,
        );
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::BUYING);
        assert_eq!(command.command_object.item_name, Some(item_name));
        assert_eq!(command.command_object.item_quantity, Some(item_quantity));
    }

    #[test]
    fn test_create_robot_purchase_command() {
        let player_id = String::from("player_id");
        let amount = 10;
        let command = Command::create_robot_purchase_command(player_id.clone(), amount);
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::BUYING);
        assert_eq!(
            command.command_object.item_name,
            Some(String::from("ROBOT"))
        );
        assert_eq!(command.command_object.item_quantity, Some(amount));
    }

    #[test]
    fn test_create_robot_upgrade_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let upgrade = RobotUpgrade::base_for_type(RobotUpgradeType::Health);
        let command =
            Command::create_robot_upgrade_command(player_id.clone(), robot_id.clone(), &upgrade);
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::BUYING);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
        assert_eq!(
            command.command_object.item_name,
            Some(upgrade.to_string_for_command())
        );
        assert_eq!(command.command_object.item_quantity, Some(1));
    }

    #[test]
    fn test_create_robot_mine_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let planet_id = String::from("9101112");
        let command = Command::create_robot_mine_command(
            player_id.clone(),
            robot_id.clone(),
            planet_id.clone(),
        );
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::MINING);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
        assert_eq!(command.command_object.planet_id, Some(planet_id));
    }

    #[test]
    fn test_create_robot_sell_inventory_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let command =
            Command::create_robot_sell_inventory_command(player_id.clone(), robot_id.clone());
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::SELLING);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
    }

    #[test]
    fn test_create_robot_regenerate_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let command = Command::create_robot_regenerate_command(player_id.clone(), robot_id.clone());
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::REGENERATE);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
    }

    #[test]
    fn test_create_robot_purchase_energy_restore_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let command = Command::create_robot_purchase_energy_restore_command(
            player_id.clone(),
            robot_id.clone(),
        );
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::BUYING);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
        assert_eq!(
            command.command_object.item_name,
            Some(String::from("\"ENERGY_RESTORE\""))
        );
        assert_eq!(command.command_object.item_quantity, Some(1));
    }

    #[test]
    fn test_create_robot_purchase_health_restore_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let command = Command::create_robot_purchase_health_restore_command(
            player_id.clone(),
            robot_id.clone(),
        );
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::BUYING);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
        assert_eq!(
            command.command_object.item_name,
            Some(String::from("\"HEALTH_RESTORE\""))
        );
        assert_eq!(command.command_object.item_quantity, Some(1));
    }

    #[test]
    fn test_create_robot_attack_command() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let target_id = String::from("9101112");
        let command = Command::create_robot_attack_command(
            player_id.clone(),
            robot_id.clone(),
            target_id.clone(),
        );
        assert_eq!(command.player_id, player_id);
        assert_eq!(command.command_type, CommandType::BATTLE);
        assert_eq!(command.command_object.robot_id, Some(robot_id));
        assert_eq!(command.command_object.target_id, Some(target_id));
    }

    #[test]
    fn test_command_object() {
        let robot_id = String::from("robot_id");
        let planet_id = String::from("planet_id");
        let target_id = String::from("target_id");
        let item_name = String::from("item_name");
        let item_quantity = 10;
        let command_object = CommandObject::new()
            .with_robot_id(robot_id.clone())
            .with_planet_id(planet_id.clone())
            .with_target_id(target_id.clone())
            .with_item_name(item_name.clone())
            .with_item_quantity(item_quantity);
        assert_eq!(command_object.robot_id, Some(robot_id));
        assert_eq!(command_object.planet_id, Some(planet_id));
        assert_eq!(command_object.target_id, Some(target_id));
        assert_eq!(command_object.item_name, Some(item_name));
        assert_eq!(command_object.item_quantity, Some(item_quantity));
    }

    #[test]
    fn test_as_json_string() {
        let player_id = String::from("1234");
        let robot_id = String::from("5678");
        let planet_id = String::from("9012");
        let command = Command::create_movement_command(
            player_id.clone(),
            robot_id.clone(),
            planet_id.clone(),
        );
        let json_string = command.as_json_string();
        assert_eq!(
            json_string,
            r#"{"playerId":"1234","type":"movement","data":{"robotId":"5678","planetId":"9012"}}"#
        );
    }
}
