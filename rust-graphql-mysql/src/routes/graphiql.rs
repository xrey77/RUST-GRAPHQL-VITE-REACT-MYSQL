use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::schema::AppSchema;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    debug_handler,
};

#[debug_handler] 
pub async fn graphiql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql_source() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
