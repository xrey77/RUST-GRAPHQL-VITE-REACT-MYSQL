use async_graphql::SimpleObject;
use async_graphql::ComplexObject;
use serde::{Serialize};
use rust_decimal::Decimal;
use sqlx::{FromRow};


#[derive(Debug, Serialize, FromRow, SimpleObject)]
pub struct ProductType {
    id: i32,
    category: String,
    descriptions: String,
    qty: i64,
    unit: String,
    costprice: Decimal,
    sellprice: Decimal,
    saleprice: Decimal,
    productpicture: String,
    alertstocks: i64,
    criticalstocks: i64
}

#[ComplexObject]
impl ProductType {} 