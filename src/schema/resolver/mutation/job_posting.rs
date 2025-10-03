use crate::{
    models::{
        address::AddressInput,
        job_posting::{ ExpectedHoursRange, ExpectedHoursRangeInput, JobPosting },
        pay::PayInput,
        prelude::*,
    },
    AppError,
    DbClient,
    Repository,
};

#[derive(Debug, Default)]
pub struct JobPostingMutation;

#[Object]
impl JobPostingMutation {
    async fn create_job_posting(
        &self,
        ctx: &Context<'_>,
        job_title: String,
        employer_name: String,
        employer_url: String,
        address: AddressInput,
        pay: Option<PayInput>,
        job_type: String,
        link_to_application: Option<String>,
        job_description: String,
        employee_responsibilities: Option<Vec<String>>,
        experience_requirements: Option<Vec<String>>,
        extra_info: Option<String>,
        expected_hours: ExpectedHoursRangeInput
    ) -> Result<JobPosting, Error> {
        info!("Creating new job posting: {}", job_title);

        let db_client = ctx.data::<DbClient>().map_err(|e| {
            warn!("Failed to get db_client from context: {:?}", e);
            AppError::InternalServerError(
                "Failed to access application db_client".to_string()
            ).to_graphql_error()
        })?;

        let id = format!("job_posting-{}", Uuid::new_v4());

        let pay_value = match pay {
            Some(p) => Some(Pay::from(p)),
            None => None,
        };

        let repo = Repository::new(db_client.clone());

        let job_posting = JobPosting::new(
            id,
            job_title,
            employer_name,
            employer_url,
            Address::from(address),
            pay_value,
            job_type,
            link_to_application,
            job_description,
            employee_responsibilities,
            experience_requirements,
            extra_info,
            ExpectedHoursRange::from(expected_hours)
        ).map_err(|e| e.to_graphql_error())?;
        
        repo.create(job_posting).await.map_err(|e| e.to_graphql_error())

    }
}
