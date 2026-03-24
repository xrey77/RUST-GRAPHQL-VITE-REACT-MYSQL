use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc}; 

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sale {
    pub id: Option<i32>,
    pub salesamount: Decimal,
    pub salesdate: DateTime<Utc>, 
}
