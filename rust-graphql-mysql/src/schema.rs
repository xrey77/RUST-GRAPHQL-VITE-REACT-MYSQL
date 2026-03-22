use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub struct QueryRoot;
#[async_graphql::Object] impl QueryRoot { async fn hello(&self) -> &str { "Hello" } }

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::new(QueryRoot, EmptyMutation, EmptySubscription)
}