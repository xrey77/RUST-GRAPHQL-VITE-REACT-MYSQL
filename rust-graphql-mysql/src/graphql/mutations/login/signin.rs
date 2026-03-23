use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use async_graphql::{Object, Context, Result, InputObject, SimpleObject, Error};
use sqlx::MySqlPool;
use bcrypt::verify;
use dotenvy::dotenv;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(InputObject)]
pub struct SigninInput {
    pub username: String,
    pub password: String
}

#[derive(SimpleObject)]
pub struct SigninResponse {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub mobile: String,
    pub isactivated: i64,
    pub isblocked: i64,
    pub mailtoken: i64,
    pub userpic: String,
    pub qrcodeurl: String,
    pub message: String,
    pub token: String
}

#[derive(Default)]
pub struct UserSignin; 

#[Object]
impl UserSignin {
    async fn signin_user(
        &self, 
        ctx: &Context<'_>, 
        input: SigninInput
    ) -> Result<SigninResponse> {
        dotenv().ok();
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let pool = ctx.data::<MySqlPool>()?;

        let user = sqlx::query!(
            "SELECT id, firstname, lastname, email, mobile, username, isactivated, isblocked, mailtoken, userpic, qrcodeurl, password_digest FROM users WHERE username = ?",
            input.username
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::new("Invalid username or password."))?; // Keep errors vague for security

        let password_hash = user.password_digest
            .as_deref()
            .ok_or_else(|| Error::new("Invalid account data."))?;

        let is_valid = verify(&input.password, password_hash)
            .map_err(|_| Error::new("Authentication system error."))?;

        if !is_valid {
            return Err(Error::new("Invalid username or password."));
        }

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        ).map_err(|_| Error::new("Token generation failed"))?;

        Ok(SigninResponse {
            id: user.id as i64,
            firstname: user.firstname.unwrap_or_default(),
            lastname: user.lastname.unwrap_or_default(),
            email: user.email,
            mobile: user.mobile.unwrap_or_default(),            
            isactivated: user.isactivated.unwrap_or(0) as i64,
            isblocked: user.isblocked.unwrap_or(0) as i64,
            mailtoken: user.mailtoken.unwrap_or(0) as i64,            
            userpic: user.userpic,
            qrcodeurl: user.qrcodeurl.unwrap_or_default(),
            message: "You have logged-in successfully.".to_string(),
            token: token
        })
    }
}


// =====REQUEST==============
// mutation SigninUser($input: SigninInput!) {
//   signinUser(input: $input) {
//     id
//     firstname
//     lastname
//     email
//     mobile
//     userpic
//     isactivated
//     isblocked
//     mailtoken
//     userpic
//     qrcodeurl
// 	  mailtoken    
//     message
//     token
//   }
// }

// ====VARIABLES=============
// {
// 	"input": {
//   	"username": "Rey",
//   	"password": "rey"
//    }
// }