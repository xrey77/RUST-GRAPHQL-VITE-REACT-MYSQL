use sqlx::FromRow;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, FromRow, async_graphql::SimpleObject)] // Added SimpleObject
pub struct Role {
    pub id: Option<i32>,
    pub name: String,
}
