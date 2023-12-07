use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImpreciseTimingDto {
    #[serde(alias="commandInputEnd")]
    command_input_ended: Option<String>,
    #[serde(alias="roundEnd")]
    round_ended: Option<String>,
    #[serde(alias="roundStart")]
    round_started: String
}
