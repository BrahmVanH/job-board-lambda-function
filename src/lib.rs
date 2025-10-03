// src/lib.rs
pub mod error;
pub mod models;
pub mod schema;
pub mod db;
pub mod repository;
pub mod config;
pub mod context;

use async_graphql::{ EmptySubscription, SchemaBuilder };
// Re-exports
pub use error::{ AppError, AppResult };
pub use models::prelude::*;
pub use repository::{ Repository, DynamoDbEntity };

use crate::schema::resolver::{ MutationRoot, QueryRoot };

// Type aliases
pub type DbClient = aws_sdk_dynamodb::Client;
pub type GraphQLSchema = async_graphql::Schema<
    schema::resolver::query::QueryRoot,
    schema::resolver::mutation::MutationRoot,
    async_graphql::EmptySubscription
>;

pub fn create_schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    use schema::resolver::{ query::QueryRoot, mutation::MutationRoot };

    async_graphql::Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        async_graphql::EmptySubscription
    )
}
