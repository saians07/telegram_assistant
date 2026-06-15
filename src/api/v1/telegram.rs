use anyhow::Result;
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use secrecy::ExposeSecret;
use teloxide::{
    payloads::SetWebhookSetters,
    prelude::{Request, Requester},
    types::{ChatId, Update, UpdateKind},
};

use crate::{
    core::{error::SwanError, middleware::request_id_generator::RequestId},
    dto::{payload::NewWebhook, response::BaseResponse},
    state::AppState,
};

#[
    utoipa::path(
        post,
        path="/api/v1/telegram/listener",
        tag="telegram",
        // request_body=Update,
        responses(
            (status=200, description="Your request has been accepted!", body=BaseResponse)
        )
    )
]
#[axum::debug_handler]
pub async fn listen(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(update): Json<Update>,
) -> Result<impl IntoResponse, SwanError> {
    // First let's get the message. For now, we only care about the message.
    // We drop anything else for now.
    let message = match update.kind {
        UpdateKind::Message(msg) => msg,
        _ => {
            return Ok(Json(BaseResponse {
                status: 400,
                message: "No messsages found!".to_string(),
                data: None,
            }));
        }
    };

    let agents = &state.bot_agents;
    let response = state
        .telegram
        .receive_message(message, agents, request_id.id)
        .await?;

    Ok(Json(response))
}

#[
    utoipa::path(
        get,
        path="/api/v1/telegram/webhook_info",
        tag="telegram",
        // request_body=Update,
        responses(
            (status=200, description="Your request has been accepted!", body=BaseResponse)
        )
    )
]
pub async fn get_webhook_info(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
) -> Result<impl IntoResponse, SwanError> {
    let webhook_info = state.telegram.bot.get_webhook_info().send().await?;

    state
        .telegram
        .bot
        .send_message(
            ChatId(state.owner_chat_id),
            format!(
                "Here is your webhook information: \n{:#?}. \nRequest ID: {}",
                webhook_info,
                request_id.as_str()
            ),
        )
        .send()
        .await?;

    Ok(Json(webhook_info))
}

#[
    utoipa::path(
        post,
        path="/api/v1/telegram/set_webhook",
        tag="telegram",
        request_body=NewWebhook,
        responses(
            (status=200, description="Your request has been accepted!", body=BaseResponse)
        )
    )
]
pub async fn set_new_webhook(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(payload): Json<NewWebhook>,
) -> Result<impl IntoResponse, SwanError> {
    state
        .telegram
        .bot
        .set_webhook(payload.into_url()?)
        .secret_token(state.secret_token.expose_secret().to_owned())
        .await?;
    state
        .telegram
        .bot
        .send_message(
            ChatId(state.owner_chat_id),
            format!(
                "Webhook has been changed to: {}\nThe secret token: {}\nRequest ID: {}",
                payload.webhook_url,
                "[redacted for security]",
                request_id.as_str()
            ),
        )
        .send()
        .await?;
    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        format!("Webhook will be changed to: {}, ", payload.webhook_url).as_str(),
    )))
}

#[
    utoipa::path(
        get,
        path="/api/v1/telegram/remove_webhook",
        tag="telegram",
        responses(
            (status=200, description="Your request has been accepted!", body=BaseResponse)
        )
    )
]
pub async fn remove_webhook(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
) -> Result<Json<BaseResponse>, SwanError> {
    state.telegram.bot.delete_webhook().await?;
    state
        .telegram
        .bot
        .send_message(
            ChatId(state.owner_chat_id),
            format!(
                "Custom webhook will be deleted! Request ID: {}",
                request_id.as_str()
            )
            .as_str(),
        )
        .send()
        .await?;

    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        "Custom webhook will be deleted!",
    )))
}
