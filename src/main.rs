use axum::{Router, serve};
use std::{collections::HashMap, env, net::SocketAddr};
use tower::ServiceBuilder;

use swan::{
    api::routes::create_routes,
    core::{
        ais::base::{BotAgent, BotInfo},
        enums::ProviderName,
        log::swan_tracing,
        middleware::request_id_generator::RequestLayer,
        traits::ai_agent::AgentTrait,
    },
    state::{self, AppState},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    swan_tracing();
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let owner_chat_id = env::var("TBOT_OWNER_ID")
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();
    let secret_token = env::var("TELEGRAM_SECRET_CODE").unwrap_or("".to_string());
    let bot = teloxide::Bot::new(bot_token);
    let mut app_state = state::AppState::new(bot, owner_chat_id, secret_token.into());
    let backend_api_routes = create_routes(app_state.clone());
    let gemini_api_key = env::var("GEMINI_API_KEY").unwrap_or("".to_string());

    // TODO: move this declaration to config.toml
    let mut bot_info = BotInfo::default();
    bot_info.bot_name = "Swan".to_string();
    bot_info.system_prompt = r#"
        You are Swan, a virtual assistant created by PT Trias Sigma Technology.
        Your jobs are as follow:
            * Engage in conversation with users about basic topics
            * Answer question related to PT TRIAS SIGMA TECHNOLOGY
            * Helping users by executing tools when possible and available.
        "#;
    bot_info.model = "gemini-3.1-flash-lite".to_string();

    // for gemini that compatible with openai API
    bot_info.base_url = "https://generativelanguage.googleapis.com/v1beta/openai".to_string();
    bot_info.api_key = gemini_api_key.clone();
    let mut ais: HashMap<String, BotAgent> = HashMap::new();
    let gemini_3_1_flash = BotAgent::new(bot_info, ProviderName::OpenAI, None).await;
    ais.insert("gemini_3_1_flash".to_string(), gemini_3_1_flash);

    let mut chat_hist = ais.get("gemini_3_1_flash").unwrap().create_history().await;
    let response = ais
        .get("gemini_3_1_flash")
        .unwrap()
        .test_func(
            "Buatin contoh kode python untuk panggil AI.",
            &mut chat_hist,
        )
        .await
        .unwrap();
    tracing::info!("AI Response {}", response);
    inject_ai_client_to_state(&mut app_state, ais).await;

    // defining the listener that will listen to our TCP
    // Deliberately pun panics here when the app start to ensure whoever starts
    // this server will get notified.
    let address = SocketAddr::from(([0, 0, 0, 0], 8300));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let tower = ServiceBuilder::new().layer(RequestLayer);

    let app = Router::new()
        .merge(backend_api_routes)
        .layer(tower)
        .with_state(app_state.clone());

    tracing::info!(
        "Starting SWAN (SMART WORKFLOW & AGENT NAVIGATOR) {}",
        address
    );

    serve(listener, app).await.unwrap();
}

async fn inject_ai_client_to_state(state: &mut AppState, mut ai: HashMap<String, BotAgent>) -> () {
    let mut state_write_guard = state.gemini_3_1_flash_lite.write().await;
    *state_write_guard = Some(ai.remove("gemini_3_1_flash").unwrap());
}
