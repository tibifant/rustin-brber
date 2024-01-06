use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PatchRoundDurationRequestBody {
    pub duration: u64,
}