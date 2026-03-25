use async_graphql::{Object, Context, Result, InputObject, SimpleObject};
use sqlx::MySqlPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::core::authenticated_users::AuthenticatedUser;

#[derive(InputObject)]
pub struct ChangePasswordInput {
    pub id: i64,
    pub password: String
}

#[derive(SimpleObject)]
pub struct ChangePasswordResponse {
    pub message: String,
}

#[derive(Default)]
pub struct ChangePassword; 

#[Object]
impl ChangePassword {
    async fn change_password(
        &self, 
        ctx: &Context<'_>, 
        input: ChangePasswordInput
    ) -> Result<ChangePasswordResponse> {

        let user = ctx.data::<AuthenticatedUser>()?;
        
        match user {
            AuthenticatedUser::User(claims) => Ok(format!("Hello user {}", claims.sub)),
            _ => Err(async_graphql::Error::new("Unauthorized")),
        }?;

        let pool = ctx.data::<MySqlPool>()?;        

        let existing = sqlx::query!(
            "SELECT email, username FROM users WHERE id = ?",
            input.id
        )
        .fetch_optional(pool)
        .await?;

        if existing.is_none () {
            return Err(async_graphql::Error::new("User not found."));
        }

        let password_hash = hash(input.password, DEFAULT_COST)
            .map_err(|_| async_graphql::Error::new("Internal server error during hashing"))?;

        sqlx::query!(
            "UPDATE users SET password_digest=? WHERE id=?",
            password_hash,
            input.id
        )
        .execute(pool)
        .await?;

        Ok(ChangePasswordResponse {
            message: "You have changed your password successfully.".to_string(),
        })
    }
}


// ======REQUEST=========
// mutation ChangePassword($input: ChangePasswordInput!) {
// 	changePassword(input: $input) {
//     message
//   }
// }

// ======VARIABLES=======
// {
//   "input": {
//     "id": 1,
//     "password": "nald"
//   }
// }