use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImpreciseTimingDto {
    #[serde(alias = "commandInputEnd")]
    pub command_input_ended: Option<String>,
    #[serde(alias = "roundEnd")]
    pub round_ended: Option<String>,
    #[serde(alias = "roundStart")]
    pub round_started: String,
}
