use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobotMovePlanetInfoDto {
    #[serde(rename = "id")]
    pub planet_id : String,
    pub movement_difficulty : u8,
}