// use sqlx::{FromRow};
// use serde::{Serialize, Deserialize};
use crate::graphql::models::role::Role;

// #[derive(Serialize, Deserialize, FromRow)]
#[derive(Clone, Debug, async_graphql::SimpleObject)] 
pub struct User {
    id: Option<i32>,
    firstname: String,
    lastname: String,
    email: String,
    mobile: String,
    username: String,
    password: String,
    roles: String,
    isactivated: i64,
    isblocked: i64,
    mailtoken: i64,
    userpic: String,
    qrcodeurl: String,
    secret: String,
    pub role: Role 
}



