use async_graphql::{Context, Object, Result, Error};
use sqlx::MySqlPool; 
use crate::graphql::types::user_types::User;
use crate::core::authenticated_users::AuthenticatedUser;

#[derive(Default)]
pub struct UsersQuery;

#[Object] 
impl UsersQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let user = ctx.data::<AuthenticatedUser>()?;
        
        match user {
            AuthenticatedUser::User(claims) => Ok(format!("Hello user {}", claims.sub)),
            _ => Err(async_graphql::Error::new("Unauthorized")),
        }?;


        let pool = ctx.data::<MySqlPool>()?;

        let users: Vec<User> = sqlx::query_as(r#"SELECT id, firstname, lastname, email, mobile, CAST(username AS CHAR) as username, isactivated, isblocked, mailtoken, userpic, COALESCE(qrcodeurl, null) as qrcodeurl FROM users"#)
        .fetch_all(pool)
        .await?;

        if users.is_empty() {
            return Err(Error::new("No record(s) found"));
        }

        Ok(users)
    }
}

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
