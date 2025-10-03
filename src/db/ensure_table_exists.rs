//! Main table creation orchestration module.
//!
//! This module provides the main entry point for ensuring all database tables exist.
//! It imports and calls the modular table creation functions from the specialized modules.

use aws_sdk_dynamodb::Client;
use crate::error::AppError;

use super::job_posting_tables;

/// Main function to ensure all required DynamoDB tables exist.
///
/// This function orchestrates the creation of all tables across different functional areas
/// by calling the specialized table creation functions from their respective modules.
///
/// # Arguments
///
/// * `client` - DynamoDB client for AWS API operations
///
/// # Returns
///
/// * `Result<(), AppError>` - Success or a database error with context
pub async fn ensure_all_tables_exist(client: &Client) -> Result<(), AppError> {
    // Get all existing tables once to avoid multiple API calls
    let tables = client
        .list_tables()
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to retrieve tables list from db client: {:?}", e.to_string())
            )
        )?;

    println!("Starting table creation process...");

    // Create job posting related tables
    println!("Creating job posting tables...");
    job_posting_tables::create_job_postings_table(&tables, client).await?;
    job_posting_tables::create_job_categories_table(&tables, client).await?;
    job_posting_tables::create_job_applications_table(&tables, client).await?;

    println!("All tables created successfully!");
    Ok(())
}
