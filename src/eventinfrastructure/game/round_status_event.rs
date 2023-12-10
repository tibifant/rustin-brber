use serde::{Deserialize, Serialize};

use crate::eventinfrastructure::game::dto::imprecise_timings_dto::ImpreciseTimingDto;
use crate::eventinfrastructure::game::dto::round_status_dto::RoundStatusDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundStatusEvent {
    pub game_id: String,
    pub round_id: String,
    pub round_number: u32,
    pub round_status: RoundStatusDto,
    pub imprecise_timing_predictions: ImpreciseTimingDto,
    pub imprecise_timings: ImpreciseTimingDto,
}




