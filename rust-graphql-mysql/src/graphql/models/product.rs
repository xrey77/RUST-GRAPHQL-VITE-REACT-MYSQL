use sqlx::{mysql::MySqlPool, FromRow};
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, FromRow)]
struct Product {
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
    criticalstocks: i64


    #[sqlx(flatten)] 
    pub category_details: Option<Category>,     
}

// To fetch a product with its category:
// let product = sqlx::query_as::<_, Product>(
//     r#"
//     SELECT p.*, c.id AS "category.id", c.name AS "category.name"
//     FROM Product p
//     JOIN Category c ON p.category_id = c.id
//     WHERE p.id = ?
//     "#
// )
// .bind(product_id)
// .fetch_one(&pool)
// .await?;
