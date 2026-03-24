use async_graphql::{Context, Object, Result, SimpleObject};
use sqlx::{MySqlPool, FromRow};
use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, FromRow, SimpleObject)]
pub struct Product {
    id: i32,
    category_id: i32,
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

#[derive(FromRow)]
pub struct Category {
    pub name: String,
}

#[Object]
impl Category {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let pool = ctx.data::<MySqlPool>()?;
        
        let products = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE category_id = (SELECT id FROM categories WHERE name = ?)"
        )
        .bind(&self.name)
        .fetch_all(pool)
        .await?;

        Ok(products)
    }
}

#[derive(Default)]
pub struct ProductCategory;

#[Object]
impl ProductCategory {
   async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<Category>> {
        let pool = ctx.data::<MySqlPool>()?;
        
        let categories = sqlx::query_as::<_, Category>(
            "SELECT DISTINCT category as name FROM products"
        )
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }    
}


// REQUEST
// query ProductByCategory {
//   categories {
//     name
//     products {
//       id
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
//   }
// }
