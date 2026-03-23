use async_graphql::SimpleObject;
use async_graphql::ComplexObject;
use serde::{Serialize, Deserialize};

#[derive(SimpleObject, Serialize, Deserialize, sqlx::FromRow)]
#[graphql(complex)] 
pub struct User {
    pub id: Option<i64>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
    pub mobile: Option<String>,
    pub username: String,
    pub isactivated: Option<i32>,
    pub isblocked: Option<i32>,
    pub mailtoken: Option<i32>,
    pub userpic: Option<String>,
    pub qrcodeurl: Option<String>
}

#[ComplexObject]
impl User {} 