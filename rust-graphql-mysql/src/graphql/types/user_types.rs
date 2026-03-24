use async_graphql::SimpleObject;
use async_graphql::ComplexObject;
use serde::{Serialize, Deserialize};

#[derive(SimpleObject, Serialize, Deserialize, sqlx::FromRow)]
#[graphql(complex)] 
pub struct User {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub mobile: String,
    pub username: String,
    pub isactivated: i64,
    pub isblocked: i64,
    pub mailtoken: i64,
    pub userpic: String,
    pub qrcodeurl: Option<String>
}

#[ComplexObject]
impl User {} 