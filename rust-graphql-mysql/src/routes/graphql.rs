use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::schema::AppSchema;

// Add these imports
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    debug_handler,
};

#[debug_handler] 
pub async fn graphql_handler(
    State(schema): State<AppSchema>, // Now State is found
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql_source() -> impl IntoResponse {
    // Now Html and IntoResponse are found
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
