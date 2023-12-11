use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CompassDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}