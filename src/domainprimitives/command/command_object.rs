use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_quantity: Option<u16>,
}

impl CommandObject {
    pub fn new() -> Self {
        CommandObject {
            robot_id: None,
            planet_id: None,
            target_id: None,
            item_name: None,
            item_quantity: None,
        }
    }

    pub fn with_robot_id(mut self, robot_id: String) -> Self {
        self.robot_id = Some(robot_id);
        self
    }
    pub fn with_planet_id(mut self, planet_id: String) -> Self {
        self.planet_id = Some(planet_id);
        self
    }

    pub fn with_target_id(mut self, target_id: String) -> Self {
        self.target_id = Some(target_id);
        self
    }

    pub fn with_item_name(mut self, item_name: String) -> Self {
        self.item_name = Some(item_name);
        self
    }

    pub fn with_item_quantity(mut self, item_quantity: u16) -> Self {
        self.item_quantity = Some(item_quantity);
        self
    }
}
