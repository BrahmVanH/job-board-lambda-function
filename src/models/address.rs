use std::collections::HashMap;

use async_graphql::InputObject;
use aws_sdk_dynamodb::types::AttributeValue;
use regex::Regex;
use serde::{ Deserialize, Serialize };

use crate::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub unit: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct AddressInput {
    pub street: String,
    pub unit: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

impl From<AddressInput> for Address {
    fn from(input: AddressInput) -> Self {
        Self {
            street: input.street,
            unit: input.unit,
            city: input.city,
            state: input.state,
            country: input.country,
            zip: input.zip,
        }
    }
}

impl From<Address> for AddressInput {
    fn from(address: Address) -> Self {
        Self {
            street: address.street,
            unit: address.unit,
            city: address.city,
            state: address.state,
            country: address.country,
            zip: address.zip,
        }
    }
}

impl Address {
    pub fn new(
        street: String,
        unit: Option<String>,
        city: String,
        state: String,
        country: String,
        zip: String
    ) -> Self {
        Self {
            street,
            unit,
            city,
            state,
            country,
            zip,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        let po_box_regex = Regex::new(
            r"(?i)^P\.?O\.?\s*Box\s+\d+|Post\s*Office\s*Box\s+\d+|Postal\s*Box\s+\d+"
        ).map_err(|e| {
            return AppError::InternalServerError(e.to_string()).to_string();
        })?;

        let street_addr_regex = Regex::new(r"^\d+\s+\w+.*").map_err(|e| {
            return AppError::InternalServerError(e.to_string()).to_string();
        })?;

        if self.street.trim().is_empty() {
            return Err("Street field cannot be empty".to_string());
        }

        if
            !street_addr_regex.is_match(self.street.trim()) &&
            !po_box_regex.is_match(self.street.trim())
        {
            return Err("Street value invalid".to_string());
        }
        if self.city.trim().is_empty() {
            return Err("City field cannot be empty".to_string());
        }
        if self.state.trim().is_empty() {
            return Err("State field cannot be empty".to_string());
        }
        if self.country.trim().is_empty() {
            return Err("Country field cannot be empty".to_string());
        }
        if self.zip.trim().is_empty() {
            return Err("Zip field cannot be empty".to_string());
        }

        Ok(())
    }

    pub(crate) fn from_attribute_value(av: &AttributeValue) -> Option<Self> {
        if let AttributeValue::M(item) = av {
            let street = item.get("street")?.as_s().ok()?.to_string();
            let unit = item.get("unit").and_then(|v| {
                match v {
                    AttributeValue::S(s) => Some(s.clone()),
                    AttributeValue::Null(_) => None,
                    _ => None,
                }
            });
            let city = item.get("city")?.as_s().ok()?.to_string();
            let state = item.get("state")?.as_s().ok()?.to_string();
            let country = item.get("country")?.as_s().ok()?.to_string();
            let zip = item.get("zip")?.as_s().ok()?.to_string();

            Some(Self {
                street,
                unit,
                city,
                state,
                country,
                zip,
            })
        } else {
            None
        }
    }
    pub(crate) fn to_attribute_value(&self) -> AttributeValue {
        let mut item = HashMap::new();

        item.insert("street".to_string(), AttributeValue::S(self.street.clone()));

        if let Some(unit) = &self.unit {
            item.insert("unit".to_string(), AttributeValue::S(unit.clone()));
        }

        item.insert("city".to_string(), AttributeValue::S(self.city.clone()));
        item.insert("state".to_string(), AttributeValue::S(self.state.clone()));
        item.insert("country".to_string(), AttributeValue::S(self.country.clone()));
        item.insert("zip".to_string(), AttributeValue::S(self.zip.clone()));
        AttributeValue::M(item)
    }
}
