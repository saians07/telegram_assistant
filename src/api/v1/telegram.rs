use anyhow::Result;
use axum::{Json, extract::State, http::StatusCode};
use teloxide::{
    prelude::{Request, Requester},
    types::{ChatId, Update},
};

use crate::{
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
pub async fn listen(
    State(state): State<AppState>,
    Json(update): Json<Update>,
) -> Result<Json<BaseResponse>, ()> {
    println!("you got new message");
    let Some(message) = update.chat() else {
        return Err(());
    };
    state
        .bot
        .send_message(message.id, "Oke, I got You!")
        .await
        .map_err(|_| ())?;
    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        "Your request has been accepted!",
    )))
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
    Json(payload): Json<NewWebhook>,
) -> Result<Json<BaseResponse>, ()> {
    println!("you got new message to change webhook");
    state
        .bot
        .set_webhook(payload.into_url()?)
        .await
        .map_err(|e| {
            println!("Error setting webhook: {:#?}", e);
            ()
        })?;
    state
        .bot
        .send_message(
            ChatId(state.owner_chat_id),
            format!("Webhook has been changed to: {}", payload.webhook_url),
        )
        .send()
        .await
        .map_err(|_| ())?;
    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        format!("Webhook will be changed to: {}", payload.webhook_url).as_str(),
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
pub async fn remove_webhook(State(state): State<AppState>) -> Result<Json<BaseResponse>, ()> {
    state.bot.delete_webhook().await.map_err(|_| ())?;
    state
        .bot
        .send_message(
            ChatId(state.owner_chat_id),
            "Custom webhook will be deleted",
        )
        .send()
        .await
        .map_err(|_| ())?;

    Ok(Json(BaseResponse::reply(
        StatusCode::OK,
        format!("Custom webhook will be deleted!").as_str(),
    )))
}
