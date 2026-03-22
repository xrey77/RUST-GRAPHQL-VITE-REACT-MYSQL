pub mod query;
pub mod mutation;
pub mod types;

use async_graphql::{Schema, EmptyMutation, EmptySubscription};
pub use query::QueryRoot;

// Convenient type alias for the full schema
pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
