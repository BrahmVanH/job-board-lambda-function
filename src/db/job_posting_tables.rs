//! Job posting system table definitions.
//!
//! This module contains table definitions for job postings, including
//! primary table and global secondary indexes for efficient querying.

use aws_sdk_dynamodb::{
    Client,
    operation::list_tables::ListTablesOutput,
    types::{
        AttributeDefinition,
        BillingMode,
        KeySchemaElement,
        KeyType,
        GlobalSecondaryIndex,
        Projection,
        ProjectionType,
        ScalarAttributeType,
    },
};

use crate::{db::common::build, error::AppError};



/// Creates the JobPostings table.
///
/// This table stores all job posting information with the following structure:
/// - Primary Key: id (String)
/// - Global Secondary Indexes:
///   - EmployerIndex: employer_name
///   - JobTypeIndex: job_type
///   - LocationIndex: address.city (for location-based queries)
///   - CreatedAtIndex: created_at (for time-based queries)
pub async fn create_job_postings_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "JobPostings";

    if tables.table_names().contains(&table_name.to_string()) {
        println!("Table '{}' already exists", table_name);
        return Ok(());
    }

    // Define attribute definitions for primary key and GSI keys
    let ad_id = build(
        AttributeDefinition::builder()
            .attribute_name("id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build id attribute definition"
    )?;

    let ad_employer_name = build(
        AttributeDefinition::builder()
            .attribute_name("employer_name")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build employer_name attribute definition"
    )?;

    let ad_job_type = build(
        AttributeDefinition::builder()
            .attribute_name("job_type")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build job_type attribute definition"
    )?;

    let ad_city = build(
        AttributeDefinition::builder()
            .attribute_name("city")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build city attribute definition"
    )?;

    let ad_created_at = build(
        AttributeDefinition::builder()
            .attribute_name("created_at")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build created_at attribute definition"
    )?;

    // Define primary key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Employer Index - for querying jobs by employer
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("employer_name").key_type(KeyType::Hash).build(),
        "Failed to build Employer GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("EmployerIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build EmployerIndex GSI"
    )?;

    // Define GSI 2: Job Type Index - for querying jobs by type (full-time, part-time, etc.)
    let gsi2_pk = build(
        KeySchemaElement::builder().attribute_name("job_type").key_type(KeyType::Hash).build(),
        "Failed to build JobType GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("JobTypeIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build JobTypeIndex GSI"
    )?;

    // Define GSI 3: Location Index - for querying jobs by city
    let gsi3_pk = build(
        KeySchemaElement::builder().attribute_name("city").key_type(KeyType::Hash).build(),
        "Failed to build Location GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("LocationIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build LocationIndex GSI"
    )?;

    // Define GSI 4: Created At Index - for querying jobs by creation time (newest first, etc.)
    let gsi4_pk = build(
        KeySchemaElement::builder().attribute_name("created_at").key_type(KeyType::Hash).build(),
        "Failed to build CreatedAt GSI PK"
    )?;

    let gsi4 = build(
        GlobalSecondaryIndex::builder()
            .index_name("CreatedAtIndex")
            .key_schema(gsi4_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build CreatedAtIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("JobPostings")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_employer_name)
        .attribute_definitions(ad_job_type)
        .attribute_definitions(ad_city)
        .attribute_definitions(ad_created_at)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .global_secondary_indexes(gsi3)
        .global_secondary_indexes(gsi4)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("JobPostings table created: {:?}", response);
    Ok(())
}

/// Creates the JobCategories table.
///
/// This table stores job categories and tags for better organization and filtering.
/// - Primary Key: id (String)
/// - Global Secondary Indexes:
///   - CategoryNameIndex: category_name (for searching by category)
pub async fn create_job_categories_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "JobCategories";

    if tables.table_names().contains(&table_name.to_string()) {
        println!("Table '{}' already exists", table_name);
        return Ok(());
    }

    // Define attribute definitions
    let ad_id = build(
        AttributeDefinition::builder()
            .attribute_name("id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build id attribute definition"
    )?;

    let ad_category_name = build(
        AttributeDefinition::builder()
            .attribute_name("category_name")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build category_name attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Category Name Index
    let gsi1_pk = build(
        KeySchemaElement::builder().attribute_name("category_name").key_type(KeyType::Hash).build(),
        "Failed to build CategoryName GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("CategoryNameIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build CategoryNameIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("JobCategories")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_category_name)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("JobCategories table created: {:?}", response);
    Ok(())
}

/// Creates the JobApplications table.
///
/// This table tracks job applications submitted through the platform.
/// - Primary Key: id (String)
/// - Global Secondary Indexes:
///   - JobPostingIndex: job_posting_id (for finding applications for a specific job)
///   - ApplicantIndex: applicant_email (for finding applications by applicant)
///   - StatusIndex: application_status (for filtering by status)
pub async fn create_job_applications_table(
    tables: &ListTablesOutput,
    client: &Client
) -> Result<(), AppError> {
    let table_name = "JobApplications";

    if tables.table_names().contains(&table_name.to_string()) {
        println!("Table '{}' already exists", table_name);
        return Ok(());
    }

    // Define attribute definitions
    let ad_id = build(
        AttributeDefinition::builder()
            .attribute_name("id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build id attribute definition"
    )?;

    let ad_job_posting_id = build(
        AttributeDefinition::builder()
            .attribute_name("job_posting_id")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build job_posting_id attribute definition"
    )?;

    let ad_applicant_email = build(
        AttributeDefinition::builder()
            .attribute_name("applicant_email")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build applicant_email attribute definition"
    )?;

    let ad_application_status = build(
        AttributeDefinition::builder()
            .attribute_name("application_status")
            .attribute_type(ScalarAttributeType::S)
            .build(),
        "Failed to build application_status attribute definition"
    )?;

    // Define key schema
    let ks_id = build(
        KeySchemaElement::builder().attribute_name("id").key_type(KeyType::Hash).build(),
        "Failed to build id key schema"
    )?;

    // Define GSI 1: Job Posting Index
    let gsi1_pk = build(
        KeySchemaElement::builder()
            .attribute_name("job_posting_id")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build JobPosting GSI PK"
    )?;

    let gsi1 = build(
        GlobalSecondaryIndex::builder()
            .index_name("JobPostingIndex")
            .key_schema(gsi1_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build JobPostingIndex GSI"
    )?;

    // Define GSI 2: Applicant Index
    let gsi2_pk = build(
        KeySchemaElement::builder()
            .attribute_name("applicant_email")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Applicant GSI PK"
    )?;

    let gsi2 = build(
        GlobalSecondaryIndex::builder()
            .index_name("ApplicantIndex")
            .key_schema(gsi2_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build ApplicantIndex GSI"
    )?;

    // Define GSI 3: Status Index
    let gsi3_pk = build(
        KeySchemaElement::builder()
            .attribute_name("application_status")
            .key_type(KeyType::Hash)
            .build(),
        "Failed to build Status GSI PK"
    )?;

    let gsi3 = build(
        GlobalSecondaryIndex::builder()
            .index_name("StatusIndex")
            .key_schema(gsi3_pk)
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build(),
        "Failed to build StatusIndex GSI"
    )?;

    // Create the table
    let response = client
        .create_table()
        .table_name("JobApplications")
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(ad_id)
        .attribute_definitions(ad_job_posting_id)
        .attribute_definitions(ad_applicant_email)
        .attribute_definitions(ad_application_status)
        .key_schema(ks_id)
        .global_secondary_indexes(gsi1)
        .global_secondary_indexes(gsi2)
        .global_secondary_indexes(gsi3)
        .send().await
        .map_err(|e|
            AppError::DatabaseError(
                format!("Failed to create {} table: {:?}", table_name, e.to_string())
            )
        )?;

    println!("JobApplications table created: {:?}", response);
    Ok(())
}
