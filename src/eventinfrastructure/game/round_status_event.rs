use serde::{Deserialize, Serialize};
use crate::eventinfrastructure::game::dto::imprecise_timings_dto::ImpreciseTimingDto;
use crate::eventinfrastructure::game::dto::round_status_dto::RoundStatusDto;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundStatusEvent {
    game_id: String,
    round_id: String,
    round_number: u32,
    round_status: RoundStatusDto,
    imprecise_timing_predictions: ImpreciseTimingDto,
    imprecise_timings: ImpreciseTimingDto,
}




