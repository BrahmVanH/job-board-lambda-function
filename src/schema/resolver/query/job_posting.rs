use crate::{
    error::AppError,
    models::{ prelude::*, job_posting::JobPosting },
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub(crate) struct JobPostingQuery;

#[Object]
impl JobPostingQuery {
    async fn job_postings(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>
    ) -> Result<Vec<JobPosting>, Error> {
        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let repo = Repository::new(db_client.clone());

        let job_postings = repo.list::<JobPosting>(limit).await.map_err(|e| e.to_graphql_error())?;

        Ok(job_postings)
    }
}
