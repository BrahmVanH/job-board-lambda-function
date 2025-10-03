use std::collections::HashMap;

use async_graphql::{ Enum, InputObject };
use aws_sdk_dynamodb::types::AttributeValue;

use chrono::{ offset::LocalResult, DateTime, TimeZone, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use tracing::info;

use crate::{ models::{ address::Address, pay::Pay }, AppError, DynamoDbEntity };

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub(crate) enum JobTypeOption {
    FullTime,
    PartTime,
    Contract,
    Temporary,
    Seasonal,
    Remote,
}

impl JobTypeOption {
    pub(crate) fn to_string(&self) -> String {
        match self {
            &JobTypeOption::FullTime => "FULL_TIME".to_string(),
            &JobTypeOption::PartTime => "PART_TIME".to_string(),
            &JobTypeOption::Contract => "CONTRACT".to_string(),
            &JobTypeOption::Temporary => "TEMPORARY".to_string(),
            &JobTypeOption::Seasonal => "SEASONAL".to_string(),
            &JobTypeOption::Remote => "REMOTE".to_string(),
        }
    }
    pub(crate) fn to_str(&self) -> &str {
        match self {
            &JobTypeOption::FullTime => "FULL_TIME",
            &JobTypeOption::PartTime => "PART_TIME",
            &JobTypeOption::Contract => "CONTRACT",
            &JobTypeOption::Temporary => "TEMPORARY",
            &JobTypeOption::Seasonal => "SEASONAL",
            &JobTypeOption::Remote => "REMOTE",
        }
    }
    pub(crate) fn from_string(s: &str) -> Result<JobTypeOption, AppError> {
        match s {
            "FULL_TIME" => Ok(Self::FullTime),
            "PART_TIME" => Ok(Self::PartTime),
            "CONTRACT" => Ok(Self::Contract),
            "TEMPORARY" => Ok(Self::Temporary),
            "SEASONAL" => Ok(Self::Seasonal),
            "REMOTE" => Ok(Self::Remote),
            _ =>
                Err(
                    AppError::DatabaseError(
                        "Cannot perform from_string on JobTypeOption input".to_string()
                    )
                ),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ExpectedHoursRange {
    pub min: u8,
    pub max: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub(crate) struct ExpectedHoursRangeInput {
    min: u8,
    max: u8,
}

impl From<ExpectedHoursRangeInput> for ExpectedHoursRange {
    fn from(input: ExpectedHoursRangeInput) -> Self {
        Self {
            min: input.min,
            max: input.max,
        }
    }
}

impl From<ExpectedHoursRange> for ExpectedHoursRangeInput {
    fn from(expected_hours: ExpectedHoursRange) -> Self {
        Self {
            min: expected_hours.min,
            max: expected_hours.max,
        }
    }
}

impl ExpectedHoursRange {
    pub(crate) fn new(min: u8, max: u8) -> Self {
        Self { min, max }
    }

    fn to_attribute_value(&self) -> AttributeValue {
        let mut item = HashMap::new();

        item.insert("min".to_string(), AttributeValue::N(self.min.to_string()));
        item.insert("max".to_string(), AttributeValue::N(self.max.to_string()));

        AttributeValue::M(item)
    }

    fn from_attribute_value(av: &AttributeValue) -> Option<Self> {
        if let AttributeValue::M(item) = av {
            let min = item.get("min")?.as_n().ok()?.parse::<u8>().ok()?;
            let max = item.get("max")?.as_n().ok()?.parse::<u8>().ok()?;

            Some(Self {
                min,
                max,
            })
        } else {
            None
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: String,
    // Job Title
    pub job_title: String,
    // Employer - linked to employer - for now just link to employer website
    pub employer_name: String,
    pub employer_url: String,
    // City, state zip
    pub address: Address,
    // hourly concat with job type (part-time, etc.)
    pub pay: Option<Pay>,
    pub job_type: JobTypeOption,
    // button linked to job application on employer site or link to closest page to applying for job
    pub link_to_application: Option<String>,

    // job details -

    // pay
    // job type (part-time, etc)
    //
    // Job description
    pub job_description: String,
    // responsibilities
    pub employee_responsibilities: Option<Vec<String>>,
    // experience
    pub experience_requirements: Option<Vec<String>>,
    // anything else the applicant should know
    pub extra_info: Option<String>,
    // expected hours - enum
    pub expected_hours: ExpectedHoursRange,
    // work location - enum

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl JobPosting {
    pub fn new(
        id: String,
        job_title: String,
        employer_name: String,
        employer_url: String,
        address: Address,
        pay: Option<Pay>,
        job_type_string: String,

        link_to_application: Option<String>,
        job_description: String,
        employee_responsibilities: Option<Vec<String>>,
        experience_requirements: Option<Vec<String>>,
        extra_info: Option<String>,
        expected_hours: ExpectedHoursRange
    ) -> Result<Self, AppError> {
        let now = Utc::now();
        let job_type = JobTypeOption::from_string(&job_type_string)?;

        Ok(Self {
            id,
            job_title,
            employer_name,
            employer_url,
            address,
            pay,
            job_type,
            link_to_application,
            job_description,
            employee_responsibilities,
            experience_requirements,
            extra_info,
            expected_hours,
            created_at: now,
            updated_at: now,
        })
    }
}

impl DynamoDbEntity for JobPosting {
    fn table_name() -> &'static str {
        "JobPostings"
    }

    fn primary_key(&self) -> String {
        self.id.clone()
    }

    fn from_item(item: &HashMap<String, AttributeValue>) -> Option<Self> {
        info!("calling from_item with: {:?}", &item);

        let id = item.get("id")?.as_s().ok()?.to_string();
        let job_title = item.get("job_title")?.as_s().ok()?.to_string();

        let employer_name = item.get("employer_name")?.as_s().ok()?.to_string();
        let employer_url = item.get("employer_url")?.as_s().ok()?.to_string();

        let address = item.get("address").and_then(|av| Address::from_attribute_value(av))?;

        let pay = item.get("pay").and_then(|av| Pay::from_attribute_value(av));

        let job_type_string = item.get("job_type")?.as_s().ok()?;
        let job_type = JobTypeOption::from_string(&job_type_string)
            .map_err(|e| e)
            .ok()?;

        let link_to_application = item
            .get("link_to_application")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let job_description = item.get("job_description")?.as_s().ok()?.to_string();

        let employee_responsibilities = item
            .get("employee_responsibilities")
            .and_then(|v| v.as_l().ok())
            .map(|list| {
                list.iter()
                    .filter_map(|av|
                        av
                            .as_s()
                            .ok()
                            .map(|s| s.to_string())
                    )
                    .collect()
            });

        let experience_requirements = item
            .get("experience_requirements")
            .and_then(|v| v.as_l().ok())
            .map(|list| {
                list.iter()
                    .filter_map(|av|
                        av
                            .as_s()
                            .ok()
                            .map(|s| s.to_string())
                    )
                    .collect()
            });

        let extra_info = item
            .get("extra_info")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.to_string());

        let expected_hours = item
            .get("expected_hours")
            .and_then(|av| ExpectedHoursRange::from_attribute_value(av))?;

        let created_at = item
            .get("created_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        let updated_at = item
            .get("updated_at")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(|| Utc::now());

        Some(Self {
            id,
            job_title,
            employer_name,
            employer_url,
            pay,
            job_type,
            link_to_application,
            job_description,
            employee_responsibilities,
            experience_requirements,
            extra_info,
            expected_hours,
            address,
            created_at,
            updated_at,
        })
    }

    fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("id".to_string(), AttributeValue::S(self.id.clone()));
        item.insert("job_title".to_string(), AttributeValue::S(self.job_title.clone()));
        item.insert("employer_name".to_string(), AttributeValue::S(self.employer_name.clone()));
        item.insert("employer_url".to_string(), AttributeValue::S(self.employer_url.clone()));
        item.insert("address".to_string(), self.address.to_attribute_value());

        // Add city as a separate field for GSI querying
        item.insert("city".to_string(), AttributeValue::S(self.address.city.clone()));

        if let Some(pay) = &self.pay {
            item.insert("pay".to_string(), pay.to_attribute_value());
        }

        item.insert("job_type".to_string(), AttributeValue::S(self.job_type.to_string()));

        if let Some(link_to_application) = &self.link_to_application {
            item.insert(
                "link_to_application".to_string(),
                AttributeValue::S(link_to_application.clone())
            );
        }

        item.insert("job_description".to_string(), AttributeValue::S(self.job_description.clone()));

        if let Some(employee_responsibilities) = &self.employee_responsibilities {
            let employee_responsibilities_list: Vec<AttributeValue> = employee_responsibilities
                .iter()
                .map(|r| AttributeValue::S(r.clone()))
                .collect();
            item.insert(
                "employee_responsibilities".to_string(),
                AttributeValue::L(employee_responsibilities_list)
            );
        }

        if let Some(experience_requirements) = &self.experience_requirements {
            let experience_requirements_list = experience_requirements
                .iter()
                .map(|r| AttributeValue::S(r.clone()))
                .collect();

            item.insert(
                "experience_requirements".to_string(),
                AttributeValue::L(experience_requirements_list)
            );
        }

        if let Some(extra_info) = &self.extra_info {
            item.insert("extra_info".to_string(), AttributeValue::S(extra_info.clone()));
        }

        item.insert("expected_hours".to_string(), self.expected_hours.to_attribute_value());
        item.insert("created_at".to_string(), AttributeValue::S(self.created_at.to_string()));
        item.insert("updated_at".to_string(), AttributeValue::S(self.updated_at.to_string()));

        item
    }
}
