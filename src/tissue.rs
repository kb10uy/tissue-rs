//! Tissue サービス側に関連する型を集約する。

use crate::checkin::Checkin;
use std::error::Error;

use chrono::prelude::*;
use serde::Deserialize;
use serde_json::{from_value, Value};

/// チェックインの成功レスポンスに含まれるチェックインデータ
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

/// Incoming Webhook のレスポンスを表す。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckinResponse {
    /// 成功
    Success(ReceivedCheckin),

    /// バリデーションエラー
    ValidationError(Vec<String>),

    /// その他のエラー
    OtherError(String),
}

/// Incoming Webhook のエンドポイントを表す。
pub struct IncomingEndpoint {
    domain: String,
    id: String,
}

impl IncomingEndpoint {
    // API ID を指定してインスタンスを生成する。
    pub fn new(id: &str) -> IncomingEndpoint {
        IncomingEndpoint {
            domain: "shikorism.net".into(),
            id: id.into(),
        }
    }

    /// ドメインと API ID を指定してインスタンスを生成する。
    pub fn with_domain(domain: &str, id: &str) -> IncomingEndpoint {
        IncomingEndpoint {
            domain: domain.into(),
            id: id.into(),
        }
    }

    /// チェックインを送信する。
    pub async fn send_checkin(
        &self,
        checkin: &Checkin,
    ) -> Result<CheckinResponse, Box<dyn Error + Send + Sync + 'static>> {
        let target_url = format!("https://{}/api/webhooks/checkin/{}", self.domain, self.id);
        let response: Value = surf::post(target_url)
            .body_json(checkin)?
            .recv_json()
            .await?;

        IncomingEndpoint::parse_response(&response)
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
                    // バリデーションエラー
                    let violations = from_value(error_object["violations"].clone())?;
                    Ok(CheckinResponse::ValidationError(violations))
                } else {
                    // その他のエラー
                    let message = error_object["message"].as_str().unwrap_or("");
                    Ok(CheckinResponse::OtherError(message.into()))
                }
            }
            otherwise => {
                Err(format!("Unknown status code: {}, response: {}", otherwise, value).into())
            }
        }
    }
}
