use async_graphql::SimpleObject;
use async_graphql::ComplexObject;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc}; 
use rust_decimal::Decimal;


#[derive(SimpleObject, Serialize, Deserialize, sqlx::FromRow)]
#[graphql(complex)] 
pub struct Sale {
    pub id: i64,
    pub salesamount: Decimal,
    pub salesdate: DateTime<Utc>,
}

#[ComplexObject]
impl Sale {} 