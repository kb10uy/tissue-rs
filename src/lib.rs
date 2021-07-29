mod checkin;
mod error;
mod tissue;

pub use crate::{
    checkin::{Checkin, CheckinBuilder},
    error::CheckinError,
    tissue::{CheckinResponse, IncomingEndpoint, ReceivedCheckin},
};

use async_trait::async_trait;
use serde_json::Value;
use std::{collections::HashMap, error::Error};

/// Trait that processes requests for Tissue.
#[async_trait]
pub trait TissueRequester {
    /// Does a GET request.
    async fn get(
        &mut self,
        url: String,
        headers: HashMap<String, String>,
    ) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Does a POST request.
    async fn post(
        &mut self,
        url: String,
        headers: HashMap<String, String>,
        body: Value,
    ) -> Result<Value, Box<dyn Error + Send + Sync>>;
}
