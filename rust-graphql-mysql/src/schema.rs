use async_graphql::{MergedObject, Schema, EmptyMutation, EmptySubscription};
use crate::graphql::queries::userid_query::UserByIdQuery;
use crate::graphql::queries::users_query::UsersQuery;
use sqlx::MySqlPool;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserByIdQuery, UsersQuery);

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(pool: MySqlPool) -> AppSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}
