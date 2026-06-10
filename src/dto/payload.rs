use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, utoipa::ToSchema, Debug)]
pub struct NewWebhook {
    pub webhook_url: String,
}

impl NewWebhook {
    pub fn into_url(&self) -> Result<Url, ()> {
        Url::parse(&self.webhook_url).map_err(|_| ())
    }
}
