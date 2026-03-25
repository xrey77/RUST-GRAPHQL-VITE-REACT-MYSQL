use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct UserWithRole {
    pub id: i32,
    pub username: String,
    pub role_id: i32,
    pub role_name: String,
}
