use sqlx::{FromRow};
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use crate::graphql::models::category::Category;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    id: Option<i32>,
    category: String,
    descriptions: String,
    qty: i64,
    unit: String,
    costprice: Decimal,
    sellprice: Decimal,
    saleprice: Decimal,
    productpicture: String,
    alertstocks: i64,
    criticalstocks: i64,


    #[sqlx(flatten)] 
    pub category_details: Option<Category>,     
}
