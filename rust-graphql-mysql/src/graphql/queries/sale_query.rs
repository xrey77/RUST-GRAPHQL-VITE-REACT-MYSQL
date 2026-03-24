use async_graphql::{Context, Object, Result, Error};
use sqlx::MySqlPool; 
use crate::graphql::types::sale_types::Sale;

#[derive(Default)]
pub struct SaleQuery;

#[Object] 
impl SaleQuery {
    async fn sale_list(&self, ctx: &Context<'_>) -> Result<Vec<Sale>> {
        let pool = ctx.data::<MySqlPool>()?;

        let sales: Vec<Sale> = sqlx::query_as(r#"SELECT id, salesamount, salesdate FROM sales"#)
        .fetch_all(pool)
        .await?;

        if sales.is_empty() {
            return Err(Error::new("No record(s) found"));
        }

        Ok(sales)
    }
}

// ===========REQUEST=========
// query SaleList{
//   saleList{
//     id
//     salesamount
//     salesdate
//   }
// }