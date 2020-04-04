use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String,
}
