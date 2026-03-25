use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
}