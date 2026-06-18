use axum::http::StatusCode;
use sea_orm::{FromJsonQueryResult, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromJsonQueryResult, FromQueryResult, Serialize, Deserialize, Default)]
pub struct TelegramHistory {
    pub author: String,
    pub message: String,
}

#[derive(serde::Serialize, utoipa::ToSchema, Debug)]
pub struct BaseResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<()>,
}

pub struct ResponseWithData<T> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

impl<T> ResponseWithData<T> {
    pub fn reply(stat: StatusCode, message: &str, data: T) -> Self {
        Self {
            status: stat.as_u16(),
            message: message.into(),
            data,
        }
    }
}

impl BaseResponse {
    pub fn reply(stat: StatusCode, message: &str) -> Self {
        Self {
            status: stat.as_u16(),
            message: message.into(),
            data: None,
        }
    }
}
