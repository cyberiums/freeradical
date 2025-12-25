// GraphQL Module

pub mod types;
pub mod query;
pub mod mutation;

use async_graphql::{Schema, EmptySubscription};
use query::QueryRoot;
use mutation::MutationRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .finish()
}
