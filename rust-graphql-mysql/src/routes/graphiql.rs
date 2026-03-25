use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::schema::AppSchema;

use axum::{
    // extract::FromRequestParts,
    // http::{request::Parts, StatusCode},
    extract::State,
    response::{Html, IntoResponse},
    // debug_handler,
};

// #[derive(Clone)]
// pub struct CurrentUser {
//     pub id: i32,
//     pub username: String,
// }

// impl<S> FromRequestParts<S> for CurrentUser
// where
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         // 1. Extract the token from headers
//         let auth_header = parts.headers
//             .get("Authorization")
//             .and_then(|v| v.to_str().ok());

//         match auth_header {
//             Some(token) if token == "valid-token" => Ok(CurrentUser {
//                 id: 1,
//                 username: "admin".to_string(),
//             }),
//             _ => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string())),
//         }
//     }
// }

// #[debug_handler] 
pub async fn graphiql_handler(
    State(schema): State<AppSchema>,
    // auth_user: Option<CurrentUser>, 
    req: GraphQLRequest,
) -> GraphQLResponse {
    // let mut request = req.into_inner();
    
    // if let Some(user) = auth_user {
    //     request = request.data(user);
    // }        
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql_source() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
