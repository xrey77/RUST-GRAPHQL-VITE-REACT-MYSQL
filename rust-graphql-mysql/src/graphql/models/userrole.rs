#[derive(Serialize, Deserialize, FromRow)]
pub struct UserWithRole {
    pub id: i32,
    pub username: String,
    pub role_id: i32,
    pub role_name: String,
}
