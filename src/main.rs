use axum::{Router, serve};
use std::{env, net::SocketAddr};

use swan::{
    api::routes::create_routes,
    state::{self},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let owner_chat_id = env::var("TBOT_OWNER_ID")
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();
    let bot = teloxide::Bot::new(bot_token);
    let app_state = state::AppState::new(bot, owner_chat_id);
    let backend_api_routes = create_routes();

    // defining the listener that will listen to our TCP
    // Deliberately pun panics here when the app start to ensure whoever starts
    // this server will get notified.
    let address = SocketAddr::from(([0, 0, 0, 0], 8300));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let app = Router::new()
        .merge(backend_api_routes)
        .with_state(app_state.clone());

    serve(listener, app).await.unwrap();
}
