use async_graphql::{Context, Object, Result, Error};
use sqlx::MySqlPool; 
use crate::graphql::types::User;

#[derive(Default)]
pub struct UsersQuery;

#[Object] 
impl UsersQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<MySqlPool>()?;

        let users = sqlx::query_as!(
            User,
            "SELECT id, firstname, lastname, email, mobile, username, isactivated, isblocked, mailtoken, userpic, qrcodeurl FROM users"
        )
        .fetch_all(pool)
        .await?;

        if users.is_empty() {
            return Err(Error::new("No record(s) found"));
        }

        Ok(users)
    }
}


// use async_graphql::{Context, Object, Result};
// use sqlx::MySqlPool; 
// use crate::graphql::types::User;

// #[derive(Default)]
// pub struct UsersQuery;

// #[Object] 
// impl UsersQuery {
//      async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
//         let pool = ctx.data::<MySqlPool>()?;

//         let users = sqlx::query_as!(
//             User,
//             "SELECT id, firstname, lastname, email, mobile, username, isactivated, isblocked, mailtoken, userpic, qrcodeurl FROM users"
//         )
//         .fetch_all(pool)
//         .await?;

        

//         Ok(users)
//     }
// }

// =====REQUEST======
// query GetUsers {
//   users{
//     id
//     firstname
//     lastname
//     email
//     mobile          
//     isactivated
//     isblocked
//     mailtoken
//     userpic
//     qrcodeurl
//   }
// }


// ====REQUEST====
// query GetUsers {
//   users{
//     id
//     firstname
//     lastname
//     email
//     mobile          
//     isactivated
//     isblocked
//     mailtoken
//     userpic
//     qrcodeurl
//   }
// }