//! Contains types corresponding Tissue service.

use crate::{checkin::Checkin, TissueRequester};
use std::{collections::HashMap, error::Error};

use chrono::prelude::*;
use serde::Deserialize;
use serde_json::{from_value, to_value, Value};

/// Returned checkin data for successful checkim request.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ReceivedCheckin {
    id: usize,
    checked_in_at: DateTime<Local>,
    note: String,
    link: String,
    tags: Vec<String>,
    source: String,
    is_private: bool,
    is_too_sensitive: bool,
}

/// Represents a response from Tissue checkin.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckinResponse {
    /// Success
    Success(ReceivedCheckin),

    /// Validation error occurred
    ValidationError(Vec<String>),

    /// Other error occurred
    OtherError(String),
}

/// Represents an endpoint for Incoming Webhook.
pub struct IncomingEndpoint<T> {
    domain: String,
    id: String,
    requester: T,
}

impl<T: TissueRequester> IncomingEndpoint<T> {
    /// Creates a new endpoint with ID.
    pub fn new(id: &str, requester: T) -> IncomingEndpoint<T> {
        IncomingEndpoint {
            domain: "shikorism.net".into(),
            id: id.into(),
            requester,
        }
    }

    /// Creates a new endpoint with domain and ID.
    pub fn with_domain(domain: &str, id: &str, requester: T) -> IncomingEndpoint<T> {
        IncomingEndpoint {
            domain: domain.into(),
            id: id.into(),
            requester,
        }
    }

    /// Sends a checkin.
    pub async fn send_checkin(
        &mut self,
        checkin: &Checkin,
    ) -> Result<CheckinResponse, Box<dyn Error + Send + Sync + 'static>> {
        let target_url = format!("https://{}/api/webhooks/checkin/{}", self.domain, self.id);
        let response: Value = self
            .requester
            .post(target_url, HashMap::new(), to_value(checkin)?)
            .await?;

        parse_response(&response)
    }
}

fn parse_response(
    value: &Value,
) -> Result<CheckinResponse, Box<dyn Error + Send + Sync + 'static>> {
    let status_code = value["status"].as_u64().expect("Status code should exist");
    match status_code {
        200 => {
            let received_checkin = from_value(value["checkin"].clone())?;
            Ok(CheckinResponse::Success(received_checkin))
        }
        404 | 422 => {
            let error_object = &value["error"];
            if error_object["violations"].is_array() {
                // Validation error
                let violations = from_value(error_object["violations"].clone())?;
                Ok(CheckinResponse::ValidationError(violations))
            } else {
                // Other error
                let message = error_object["message"].as_str().unwrap_or("");
                Ok(CheckinResponse::OtherError(message.into()))
            }
        }
        otherwise => {
            Err(format!("Unknown status code: {}, response: {}", otherwise, value).into())
        }
    }
}
