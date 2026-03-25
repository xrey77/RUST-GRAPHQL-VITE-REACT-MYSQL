use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::schema::AppSchema;
use crate::core::authenticated_users::AuthenticatedUser;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

#[axum::debug_handler] 
pub async fn graphiql_handler(
    State(schema): State<AppSchema>,
    user: AuthenticatedUser,
    req: GraphQLRequest,
) -> GraphQLResponse {

    let mut req = req.into_inner();
    req = req.data(user);
    schema.execute(req).await.into()    

}

pub async fn graphiql_source() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
