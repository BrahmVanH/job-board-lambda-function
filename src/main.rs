use std::env;

use aws_config::Region;
use axum::{ extract::Extension, http::{ HeaderValue, Method }, routing::get, Router };
use dotenvy::dotenv;
use job_board_lambda::{
    config::Config,
    context::{ AppContext, ContextExtensions },
    create_schema,
    db,
    AppError,
    DbClient,
    GraphQLSchema,
};
use tower::ServiceBuilder;
use tower_http::{ compression::CompressionLayer, cors::{ Any, CorsLayer } };
use async_graphql_axum::{ GraphQLRequest, GraphQLResponse };
use serde::Serialize;
use tracing::{ info, error };

// mod auth;

fn main() {
    println!("Hello, world!");
}
