use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RegisterPlayerRequestBody {
    pub name: String,
    pub email: String,
}
