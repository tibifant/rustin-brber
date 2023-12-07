use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CompassDirectionDto {
    NORTH,
    EAST,
    SOUTH,
    WEST
}