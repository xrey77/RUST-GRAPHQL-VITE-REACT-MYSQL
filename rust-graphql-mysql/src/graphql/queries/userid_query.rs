use async_graphql::{Context, Object, Result, ID, Error};
use sqlx::MySqlPool; 
use crate::graphql::types::User;

#[derive(Default)]
pub struct UserByIdQuery;

#[Object] 
impl UserByIdQuery {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<User>> {
        let pool = ctx.data::<MySqlPool>()?;

        let user_id = id.parse::<i64>()?;
        let user = sqlx::query_as::<_, User>(
            "SELECT id, firstname, lastname, email, mobile, username, isactivated, isblocked, mailtoken, userpic, qrcodeurl 
            FROM users u WHERE id = ?"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;        

        match user {
            Some(u) => Ok(Some(u)),
            None => Err(Error::new("No record(s) found")),
        }        
    }
}

// =======REQUEST===============
// query GetUserId($id: ID!) {
//   user(id: $id) {
//   	id
//     firstname
//     lastname
//     email
//     mobile
//     isactivated
//     isblocked
//     userpic
//     qrcodeurl
//   }
  
// }

// =======VARIABLES=============
// {
//   "id": "1"
// }