use anyhow::Result;
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use secrecy::ExposeSecret;
use teloxide::{
    payloads::{SendVoiceSetters, SetWebhookSetters},
    prelude::{Request, Requester},
    types::{ChatId, InputFile, Update, UpdateKind},
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
    // Extension(request_id): Extension<RequestId>,
    Json(update): Json<Update>,
) -> Result<impl IntoResponse, SwanError> {
    // First let's get the message. For now, we only care about the message.
    // We drop anything else for now.
    let message = match update.kind {
        UpdateKind::Message(msg) => msg,
        _ => return Err(SwanError::operation("No message found")),
    };

    // To avoid impostor, we remove anyone other than trusted people
    // Currently, we will use this simple approach.
    // TODO: Use database later!
    if message.chat.id.0 != state.owner_chat_id {
        let audio_file = InputFile::file("assets/voices/Belum_Terdaftar.mp3");
        let _ = state
            .bot
            .send_voice(message.chat.id, audio_file)
            .caption("Pesan dari Tarzan!")
            .await?;
    }
    let _ = state
        .bot
        .send_message(message.chat.id, "Oke, I got you!")
        .await?;

    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        "Your request has been accepted!",
    )))
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
    let webhook_info = state.bot.get_webhook_info().send().await?;

    state
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
        .bot
        .set_webhook(payload.into_url()?)
        .secret_token(state.secret_token.expose_secret().to_owned())
        .await
        .map_err(|e| e)?;
    state
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
) -> Result<Json<BaseResponse>, ()> {
    state.bot.delete_webhook().await.map_err(|_| ())?;
    state
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
        .await
        .map_err(|_| ())?;

    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        format!("Custom webhook will be deleted!").as_str(),
    )))
}
