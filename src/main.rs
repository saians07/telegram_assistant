use axum::{Json, Router, extract::State, http::StatusCode, routing::post, serve};
use std::{env, net::SocketAddr};
use teloxide::prelude::*;

use bella_swan::state::{self, AppState};

#[tokio::main]
async fn main() {
    let bot = teloxide::Bot::from_env();
    let owner_chat_id = env::var("OWNER_CHAT_ID")
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();
    let app_state = state::AppState::new(bot, owner_chat_id);

    // defining the listener that will listen to our TCP
    // Deliberately pun panics here when the app start to ensure whoever starts
    // this server will get notified.
    let address = SocketAddr::from(([0, 0, 0, 0], 8300));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let app: Router = Router::new()
        .route("/webhook", post(telegram_handler))
        .with_state(app_state.clone());

    serve(listener, app).await.unwrap();
}

async fn telegram_handler(
    State(state): State<AppState>,
    Json(update): Json<Update>,
) -> Result<axum::http::StatusCode, ()> {
    println!("You got a new messages: {:#?}", update);
    let Some(message) = update.chat() else {
        return Ok(StatusCode::BAD_REQUEST);
    };
    let a = state
        .bot
        .send_message(message.id, "You are sending me a message")
        .await
        .map_err(|e| print!("Error: {:#?}", e))?;

    print!("{:#?}", a);
    Ok(axum::http::StatusCode::OK)
}
