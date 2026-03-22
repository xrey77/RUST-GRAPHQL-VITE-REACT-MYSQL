use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc}; 

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sale {
    pub id: Option<i32>,
    pub salesamount: Decimal,
    pub salesdata: DateTime<Utc>, 
}

// use sqlx::{mysql::MySqlPool, FromRow};
// use serde::{Serialize, Deserialize};
// use rust_decimal::Decimal;

// #[macro_use]
// extern crate serde_derive;

// #[derive(Serialize, Deserialize, FromRow)]
// struct Sale {
//     id: Option<i32>,
//     salesamount: Decimal,
//     salesdata: DateTime,
// }