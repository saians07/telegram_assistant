use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::core::error::SwanError;

#[derive(Serialize, Deserialize, utoipa::ToSchema, Debug)]
pub struct NewWebhook {
    pub webhook_url: String,
}

impl NewWebhook {
    pub fn into_url(&self) -> Result<Url, SwanError> {
        Url::parse(&self.webhook_url).map_err(|e| {
            SwanError::operation_with_source("Failed to convert new webhook URL to a URL type!", e)
        })
    }
}
