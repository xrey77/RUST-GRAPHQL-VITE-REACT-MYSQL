use async_graphql::{Context, Object, Result, Error, SimpleObject};
use sqlx::{MySqlPool, FromRow};
use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, FromRow, SimpleObject)]
pub struct ProductSearched {
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
pub struct ProductSearchResponse {
    products: Vec<ProductSearched>,
    totpage: u32,
    totalrecords: i64,
    page: i32,
}

#[derive(Default)]
pub struct ProductSearch;

#[Object] 
impl ProductSearch {
    async fn product_search(&self, ctx: &Context<'_>, page: i32, keyword: String) -> Result<ProductSearchResponse> {
        let pool = ctx.data::<MySqlPool>()?;
        let search_pattern = format!("%{}%", keyword);


        let (totalcount,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE descriptions LIKE ?")
            .bind(&search_pattern)
            .fetch_one(pool)
            .await?;
        
        let per_page = 5;
        let offset = (page - 1) * per_page;
        let total_pages = (totalcount as f64 / per_page as f64).ceil() as u32;

        let products_result: Vec<ProductSearched> = sqlx::query_as(r#"SELECT id,category,descriptions,qty,unit,costprice,sellprice,saleprice,productpicture,alertstocks,criticalstocks FROM products WHERE descriptions LIKE ? LIMIT ?, ?"#)
        .bind(&search_pattern)
        .bind(&offset)
        .bind(&per_page)
        .fetch_all(pool)
        .await?;


        if products_result.is_empty() {
            return Err(Error::new("No record(s) found"));
        }

        Ok(ProductSearchResponse {
            products: products_result,
            totpage: total_pages,
            totalrecords: totalcount,
            page: page,
        })
    }
}

// =============REQUEST==============
// query ProductSearch($page: Int!, $keyword: String!) {
//   productSearch(page: $page, keyword: $keyword) {
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

// =======VARIABLES========
// {
//   "page": 1,
//   "keyword": "cineo"
// }