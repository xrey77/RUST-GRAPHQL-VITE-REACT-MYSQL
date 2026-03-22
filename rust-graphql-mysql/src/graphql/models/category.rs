use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc}; 

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
}