use async_graphql::{Object, Context, Result, InputObject, SimpleObject};
use sqlx::MySqlPool;
use totp_rs::{Algorithm, TOTP};
use base32::Alphabet;
use sqlx::{FromRow};
use serde::Serialize;

#[derive(InputObject)]
pub struct OtpVerificationInput {
    pub id: Option<i64>,
    pub otp: String,
}

#[derive(SimpleObject)]
pub struct OtpVerificationResponse {
    pub username: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Users {
    username: Option<String>,
    email: String,
    secret: Option<String>,
}
#[derive(Default)]  
pub struct OtpVerification;

#[Object]
impl OtpVerification {
    async fn otp_verification(
        &self, 
        ctx: &Context<'_>, 
        input: OtpVerificationInput
    ) -> Result<OtpVerificationResponse> {
        let pool = ctx.data::<MySqlPool>()?;

        let user = sqlx::query_as::<_, Users>("SELECT email, CAST(username AS CHAR) as username, secret FROM users WHERE id = ?")
            .bind(input.id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| async_graphql::Error::new("User not found."))?;

        let secret_str = user.secret.filter(|s| !s.is_empty())
            .ok_or_else(|| async_graphql::Error::new("MFA is not yet activated."))?;

        let secret_bytes = base32::decode(Alphabet::Rfc4648 { padding: true }, &secret_str)
            .ok_or_else(|| async_graphql::Error::new("Internal server error: Invalid secret format."))?;

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
            Some("WORLD BANK".to_string()),
            user.email,
        ).map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let valid = totp.check_current(&input.otp)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if valid {
            Ok(OtpVerificationResponse {
                username: user.username.clone(), 
                message: "Successful OTP code verification.".to_string(),
            })
        } else {
            Err(async_graphql::Error::new("Invalid OTP code, please try again."))
        }
    }
}

// REQUEST
// mutation OtpVerification($input: OtpVerificationInput!) {
//   otpVerification(input: $input) {
//     username
//     message
//   }
// }


// VARIABLES
// {
//   "input": {
//     "id": 1,
//     "otp": "2342343"
//   }
// }



