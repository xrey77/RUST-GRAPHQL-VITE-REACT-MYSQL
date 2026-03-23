use async_graphql::{Object, Context, Result, InputObject, SimpleObject};
use sqlx::MySqlPool;
use totp_rs::{Algorithm, TOTP, Secret};
use data_encoding::BASE32;

#[derive(InputObject)]
pub struct ActivateMfaInput {
    pub id: i64,
    pub twofactorenabled: bool,
}

#[derive(SimpleObject)]
pub struct ActivateMfaResponse {
    pub qrcodeurl: Option<String>,
    pub message: String,
}

#[derive(Default)]
pub struct ActivateMfa; 

#[Object]
impl ActivateMfa {
    async fn activate_mfa(
        &self, 
        ctx: &Context<'_>, 
        input: ActivateMfaInput
    ) -> Result<ActivateMfaResponse> {
        let pool = ctx.data::<MySqlPool>()?;


        let users_result = sqlx::query!(
            "SELECT email, username FROM users WHERE id = ?",
            input.id
        )
        .fetch_optional(pool)
        .await?;

        let Some(user) = users_result else {
            return Err(async_graphql::Error::new("User not found."));
        };

        if input.twofactorenabled {

        let secret: Secret = Secret::generate_secret();

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                secret.to_bytes().expect("Invalid secret bytes"),
                Some("WORLD BANK".to_string()),
                user.email,
            ).unwrap();        

            let qrcode_base64 = totp.get_qr_base64();
            let qrcode_string: String = qrcode_base64.clone().expect("Failed to get the base64 string");

            let secret_bytes: Vec<u8> = secret.to_bytes().expect("Failed to convert secret to bytes");
        
            let encoded_secret: String = BASE32.encode(&secret_bytes);

            sqlx::query!(
              "UPDATE users SET secret = ?, qrcodeurl = ? WHERE id = ?",
              encoded_secret,
              qrcode_string,
              input.id
            ).execute(pool).await?;

            Ok(ActivateMfaResponse {
                qrcodeurl: Some(qrcode_string),
                message: "Multi-Factor Authenticator enabled successfully.".to_string(),
            })
        } else {
            sqlx::query!(
              "UPDATE users SET secret = NULL, qrcodeurl = NULL WHERE id = ?",
              input.id
            ).execute(pool).await?;

            Ok(ActivateMfaResponse {
                qrcodeurl: None,
                message: "Multi-Factor Authenticator disabled successfully.".to_string(),
            })
        }
    }
}


// ======REQUEST=======
// mutation ActivateMfa($input: ActivateMfaInput!) {
//   activateMfa(input: $input) {
//     qrcodeurl
//     message
//   }
// }

// =======VARIABLES=======
// {
//   "input": {
//     "id": 1,
//     "twofactorenabled": false
//   }
// }