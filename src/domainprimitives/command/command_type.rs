use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommandType {
    MOVEMENT,
    BATTLE,
    MINING,
    REGENERATE,
    BUYING,
    SELLING,
}
