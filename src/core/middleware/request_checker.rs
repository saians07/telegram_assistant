/*
 * This comes before any request touch our telegram webhook endpoint
 * This will help us from not only DDos but also unauthorized bot access.
 * Since this will be very simple check against our token, it will be implemented
 * using `from_fn_with_state`.
 */

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use secrecy::ExposeSecret;

use crate::{core::error::SwanError, state::AppState};

pub async fn request_checker(State(state): State<AppState>, req: Request, next: Next) -> Response {
    const TELEGRAM_TOKEN_HEADER: &str = "X-Telegram-Bot-Api-Secret-Token";

    let token = req
        .headers()
        .get(TELEGRAM_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok());

    match token {
        Some(t) if t == state.secret_token.expose_secret() => next.run(req).await,
        _ => {
            tracing::error!("Token do not match: {}", SwanError::UnauthorizedBot);
            StatusCode::OK.into_response()
        }
    }
}
