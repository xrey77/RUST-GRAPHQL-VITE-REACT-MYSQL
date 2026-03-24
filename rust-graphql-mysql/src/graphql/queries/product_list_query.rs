use async_graphql::{Context, Object, Result, Error, SimpleObject};
use sqlx::{MySqlPool, FromRow};
use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, FromRow, SimpleObject)]
pub struct Products {
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

#[derive(SimpleObject)]
pub struct ProductListResponse {
    products: Vec<Products>,
    totpage: u32,
    totalrecords: i64,
    page: i32,
}

#[derive(Default)]
pub struct ProductList;

#[Object] 
impl ProductList {
    async fn product_list(&self, ctx: &Context<'_>, page: i32) -> Result<ProductListResponse> {
        let pool = ctx.data::<MySqlPool>()?;

        let (total_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
            .fetch_one(pool)
            .await?;
        
        let per_page = 5;
        let offset = (page - 1) * per_page;
        let total_pages = (total_count as f64 / per_page as f64).ceil() as u32;

        let products_result: Vec<Products> = sqlx::query_as(r#"SELECT id,category,descriptions,qty,unit,costprice,sellprice,saleprice,productpicture,alertstocks,criticalstocks FROM products LIMIT ?, ?"#)
        .bind(&offset)
        .bind(&per_page)
        .fetch_all(pool)
        .await?;


        if products_result.is_empty() {
            return Err(Error::new("No record(s) found"));
        }

        Ok(ProductListResponse {
            products: products_result,
            totpage: total_pages,
            totalrecords: total_count,
            page: page,
        })
    }
}

// ======REQUEST=================
// query ProductList($page: Int!) {
//   productList(page: $page) {
//     products {
//       id
//       category
//       descriptions
//       qty
//       unit
//       costprice
//       sellprice
//       saleprice
//       productpicture
//       alertstocks
//       criticalstocks
//     }    
//     page
//     totpage
//     totalrecords
//   }
// }

// =======VARIABLES======
// {
//   "page": 2
// }