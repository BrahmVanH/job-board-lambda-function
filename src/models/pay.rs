use std::collections::HashMap;

use async_graphql::{ Enum, InputObject };
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{ Deserialize, Serialize };

use crate::AppError;

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CadenceOption {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl CadenceOption {
    pub(crate) fn to_string(&self) -> String {
        match self {
            &CadenceOption::Hour => "HOUR".to_string(),
            &CadenceOption::Day => "DAY".to_string(),
            &CadenceOption::Week => "WEEK".to_string(),
            &CadenceOption::Month => "MONTH".to_string(),
            &CadenceOption::Year => "YEAR".to_string(),
        }
    }
    pub(crate) fn to_str(&self) -> &str {
        match self {
            &CadenceOption::Hour => "HOUR",
            &CadenceOption::Day => "DAY",
            &CadenceOption::Week => "WEEK",
            &CadenceOption::Month => "MONTH",
            &CadenceOption::Year => "YEAR",
        }
    }
    pub(crate) fn from_string(s: &str) -> Result<CadenceOption, AppError> {
        match s {
            "HOUR" => Ok(Self::Hour),
            "DAY" => Ok(Self::Day),
            "WEEK" => Ok(Self::Week),
            "MONTH" => Ok(Self::Month),
            "YEAR" => Ok(Self::Year),
            _ =>
                Err(
                    AppError::DatabaseError(
                        "Cannot perform from_string on CadenceOption input".to_string()
                    )
                ),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pay {
    pub cadence: CadenceOption,
    pub min_base_pay: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct PayInput {
    pub cadence: CadenceOption,
    pub min_base_pay: u32,
}

impl From<PayInput> for Pay {
    fn from(input: PayInput) -> Self {
        Self {
            cadence: input.cadence,
            min_base_pay: input.min_base_pay,
        }
    }
}

impl From<Pay> for PayInput {
    fn from(pay: Pay) -> Self {
        Self {
            cadence: pay.cadence,
            min_base_pay: pay.min_base_pay,
        }
    }
}

impl Pay {
    pub fn new(cadence: String, min_base_pay: u32) -> Result<Self, AppError> {
        let cadence_option = CadenceOption::from_string(&cadence)?;

        Ok(Self {
            cadence: cadence_option,
            min_base_pay,
        })
    }

    pub(crate) fn from_attribute_value(av: &AttributeValue) -> Option<Self> {
        if let AttributeValue::M(item) = av {
            let cadence_string = item.get("cadence")?.as_s().ok()?;
            let cadence = CadenceOption::from_string(&cadence_string)
                .map_err(|e| e)
                .ok()?;

            let min_base_pay_string = item.get("min_base_pay")?.as_s().ok()?;
            let min_base_pay = min_base_pay_string.parse::<u32>().ok()?;

            Some(Self {
                cadence,
                min_base_pay,
            })
        } else {
            None
        }
    }

    pub(crate) fn to_attribute_value(&self) -> AttributeValue {
        let mut item = HashMap::new();

        item.insert("cadence".to_string(), AttributeValue::S(self.cadence.to_string()));
        item.insert("min_base_pay".to_string(), AttributeValue::S(self.min_base_pay.to_string()));

        AttributeValue::M(item)
    }
}
