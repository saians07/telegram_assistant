use std::{borrow::Cow, error::Error};

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dto::response::BaseResponse;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SwanError {
    #[error("Database operation failed")]
    #[serde(skip)]
    Database(#[from] sea_orm::DbErr),

    #[serde(skip)]
    #[error("Transaction failed: {source}")]
    Transaction {
        #[source]
        source: sea_orm::DbErr,
        operation: &'static str,
    },

    #[error("Telegram Bot Failed: {0}")]
    #[serde(skip)]
    TelegramRequestError(#[from] teloxide::RequestError),

    #[error("Unknown bot!")]
    UnauthorizedBot,

    #[error("No record found {0}")]
    NotFoundError(String),

    #[error("The session has been expired.")]
    SessionEnded,

    #[error("Environment key: {0} not found!")]
    MissingEnvVar(String),

    #[error("Operation failed: {operation}")]
    #[serde(skip)]
    Operation {
        operation: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn Error + Send + Sync>>,
    },
}

impl SwanError {
    pub fn operation_with_source<S: Into<Cow<'static, str>>, E: Error + Send + Sync + 'static>(
        operation: S,
        source: E,
    ) -> Self {
        SwanError::Operation {
            operation: operation.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn operation<S>(operation: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        SwanError::Operation {
            operation: operation.into(),
            source: None,
        }
    }

    fn user_message(&self) -> &'static str {
        match self {
            SwanError::TelegramRequestError(_) => "Telegram bot failed to process!",
            _ => "",
        }
    }
}

impl IntoResponse for SwanError {
    fn into_response(self) -> Response {
        tracing::error!(
            error_details = %self.to_string(),
            error_source = ?Error::source(&self),
            "Request failed"
        );

        let status = match self {
            SwanError::TelegramRequestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SwanError::UnauthorizedBot => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(BaseResponse {
            status: status.into(),
            message: self.user_message().to_string(),
            data: None,
        });

        (status, body).into_response()
    }
}
