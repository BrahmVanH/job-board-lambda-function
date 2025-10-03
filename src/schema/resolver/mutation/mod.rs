use async_graphql::MergedObject;

pub mod job_posting;

#[derive(Debug, Default, MergedObject)]
pub struct MutationRoot(job_posting::JobPostingMutation);
