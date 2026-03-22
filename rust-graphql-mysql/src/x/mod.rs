use async_graphql::{Object, Schema, EmptySubscription, EmptyMutation};

// 1. Define your Query root
pub struct Query;

#[Object]
impl Query {
    // You must have at least one field in your Query root
    async fn version(&self) -> &str {
        "1.0"
    }
}

// 2. Define your Mutation root (or use EmptyMutation)
pub struct Mutation;

#[Object]
impl Mutation {
    async fn test_mutation(&self, input: String) -> String {
        input
    }
}

// 3. Now your AppSchema type will find the definitions
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;



// use async_graphql::Schema;

// pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

// pub fn build_schema() -> AppSchema {
//     Schema::build(Query, Mutation, EmptySubscription).finish()
// }
