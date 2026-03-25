use async_graphql::{Object, Context, Result, InputObject, SimpleObject};
use sqlx::MySqlPool;
use crate::core::authenticated_users::AuthenticatedUser;

#[derive(InputObject)]
pub struct ProfileInput {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub mobile: String
}

#[derive(SimpleObject)]
pub struct ProfileResponse {
    pub message: String,
}

#[derive(Default)]
pub struct ProfileMutation; 

#[Object]
impl ProfileMutation {
    async fn profile_update(
        &self, 
        ctx: &Context<'_>, 
        input: ProfileInput
    ) -> Result<ProfileResponse> {

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

        sqlx::query!(
            "UPDATE users SET firstname=?, lastname=?, mobile=? WHERE id=?",
            input.firstname,
            input.lastname,
            input.mobile,
            input.id
        )
        .execute(pool)
        .await?;

        Ok(ProfileResponse {
            message: "You have updated your profile successfully.".to_string(),
        })
    }
}


// ====REQUEST============
// mutation UpdateProfile($input: ProfileInput!) {
//   profileUpdate(input: $input){
//     message
//   }
// }


// =====VARIABLES========
// {
//   "input": {
//     "id": 1,
//     "firstname": "Reynaldo",
//     "lastname": "Marquez",
//     "mobile": "23423423"
//   }
// }