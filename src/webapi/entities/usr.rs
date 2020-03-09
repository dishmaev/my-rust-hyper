use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String,
}
