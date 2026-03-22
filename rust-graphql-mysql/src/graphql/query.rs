use async_graphql::{Object, Context};
use crate::schema::types::User;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, _ctx: &Context<'_>) -> User {
        User { id: 1, username: "rust_ace".to_string() }
    }
}
