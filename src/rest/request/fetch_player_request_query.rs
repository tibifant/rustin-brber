use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct FetchPlayerRequestQuery {
    pub name: String,
    #[serde(rename = "mail")]
    pub email: String,
}
