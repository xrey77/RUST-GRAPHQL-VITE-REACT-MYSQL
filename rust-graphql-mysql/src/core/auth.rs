use crate::core::authenticated_users::AuthenticatedUser;
use crate::graphql::models::claims_struct::Claims;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts},
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Try to get the header
        let auth_header = match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
            Ok(header) => header,
            Err(_) => return Ok(AuthenticatedUser::Anonymous),
        };

        // 2. Try to decode the token
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes());
        
        match jsonwebtoken::decode::<Claims>(auth_header.token(), &decoding_key, &jsonwebtoken::Validation::default()) {
            Ok(token_data) => Ok(AuthenticatedUser::User(token_data.claims)),
            Err(_) => Ok(AuthenticatedUser::Anonymous),
        }
    }
}

