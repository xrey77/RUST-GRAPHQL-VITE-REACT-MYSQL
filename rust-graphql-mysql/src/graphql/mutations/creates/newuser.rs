use async_graphql::{Object, Context, Result, InputObject, SimpleObject};
use sqlx::MySqlPool;
use bcrypt::{hash, DEFAULT_COST};

#[derive(InputObject)]
pub struct RegistrationInput {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub mobile: String,
    pub username: String,
    pub password: String
}

#[derive(SimpleObject)]
pub struct RegistrationResponse {
    pub message: String,
}

#[derive(Default)]
pub struct FormRegistration; 

#[Object]
impl FormRegistration {
    async fn register_user(
        &self, 
        ctx: &Context<'_>, 
        input: RegistrationInput
    ) -> Result<RegistrationResponse> {
        let pool = ctx.data::<MySqlPool>()?;
        

        let existing = sqlx::query!(
            "SELECT email, username FROM users WHERE email = ? OR username = ?",
            input.email,
            input.username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user) = existing {
            let existing_username = String::from_utf8_lossy(&user.username);

            if user.email == input.email {
                return Err(async_graphql::Error::new("Email Address is already taken."));
            }

            if existing_username == input.username {
                return Err(async_graphql::Error::new("Username is already taken."));
            }
        }

        let password_hash = hash(input.password, DEFAULT_COST)
            .map_err(|_| async_graphql::Error::new("Internal server error during hashing"))?;

        sqlx::query!(
            "INSERT INTO users (firstname, lastname, email, mobile, username, password_digest, role_id) VALUES (?, ?, ?, ?, ?, ?, 2)",
            input.firstname,
            input.lastname,
            input.email,
            input.mobile,
            input.username,
            password_hash
        )
        .execute(pool)
        .await?;

        Ok(RegistrationResponse {
            message: "You have registered successfully, please login now.".to_string(),
        })
    }
}

// =====REQUEST==============
// mutation RegisterUser($input: RegistrationInput!) {
//   registerUser(input: $input) {
//     message
//   }
// }


// ====VARIABLES=============
// {
// 	"input": {
//     "firstname": "Rey",
//     "lastname": "Gragasin",
//     "email": "rey@yahoo.com",
//     "mobile": "23423423",
//   	"username": "Rey",
//   	"password": "rey"
//    }
// }