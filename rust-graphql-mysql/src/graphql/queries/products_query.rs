use async_graphql::{Context, Object, Result, Error};
use sqlx::MySqlPool; 
use crate::graphql::types::product_types::ProductType;

#[derive(Default)]
pub struct ProductReport;

#[Object] 
impl ProductReport {
    async fn product_report(&self, ctx: &Context<'_>) -> Result<Vec<ProductType>> {
        let pool = ctx.data::<MySqlPool>()?;

        let products: Vec<ProductType> = sqlx::query_as(r#"SELECT id, category, descriptions, qty, unit, costprice, sellprice, saleprice, productpicture, alertstocks, criticalstocks FROM products"#)
        .fetch_all(pool)
        .await?;

        if products.is_empty() {
            return Err(Error::new("No record(s) found"));
        }

        Ok(products)
    }
}

// ======REQUEST========
// query ProductReport {
// 	productReport{
//     id
//     category
//     descriptions
//     qty
//     unit
//     costprice
//     sellprice
//     saleprice
//     productpicture
//     alertstocks
//     criticalstocks
//   }
// }