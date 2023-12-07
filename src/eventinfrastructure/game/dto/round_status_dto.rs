use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RoundStatusDto {
    Started,
    #[serde(rename = "command input ended")]
    CommandInputEnded,
    Ended
}