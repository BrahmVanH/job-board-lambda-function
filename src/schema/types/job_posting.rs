use crate::models::{ prelude::*, job_posting::{ JobTypeOption, ExpectedHoursRange } };

#[Object]
impl JobPosting {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn job_title(&self) -> &str {
        &self.job_title
    }
    async fn employer_name(&self) -> &str {
        &self.employer_name
    }
    async fn employer_url(&self) -> &str {
        &self.employer_url
    }
    async fn address(&self) -> &Address {
        &self.address
    }
    async fn pay(&self) -> &Option<Pay> {
        &self.pay
    }
    async fn job_type(&self) -> &JobTypeOption {
        &self.job_type
    }
    async fn link_to_application(&self) -> &Option<String> {
        &self.link_to_application
    }
    async fn job_description(&self) -> &str {
        &self.job_description
    }
    async fn employee_responsibilities(&self) -> &Option<Vec<String>> {
        &self.employee_responsibilities
    }
    async fn experience_requirements(&self) -> &Option<Vec<String>> {
        &self.experience_requirements
    }
    async fn extra_info(&self) -> &Option<String> {
        &self.extra_info
    }
    async fn expected_hours(&self) -> &ExpectedHoursRange {
        &self.expected_hours
    }
    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[Object]
impl ExpectedHoursRange {
    async fn min(&self) -> &u8 {
        &self.min
    }

    async fn max(&self) -> &u8 {
        &self.max
    }
}
