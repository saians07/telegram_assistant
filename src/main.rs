use anyhow::Result;
use axum::{Router, serve};
use sea_orm::{Database, DatabaseConnection};
use std::{collections::HashMap, env, net::SocketAddr};
use tower::ServiceBuilder;

use swan::{
    api::routes::create_routes,
    core::{
        ais::base::{BotAgent, BotInfo},
        config::DatabaseConfig,
        enums::ProviderName,
        error::SwanError,
        log::swan_tracing,
        middleware::request_id_generator::RequestLayer,
    },
    repositories::telegram::TelegramRepo,
    service::telegram::TelegramService,
    state::{self},
};

#[tokio::main]
async fn main() -> Result<(), SwanError> {
    dotenvy::dotenv().ok();
    swan_tracing();

    // database related setting
    let db_config = DatabaseConfig::new()?;
    let db_con: DatabaseConnection =
        Database::connect(db_config.create_options("Postgres".to_string()).await?).await?;
    run_migrations(&db_con).await?;

    // telegram setting// TODO: let's move this to constant
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let owner_chat_id = env::var("TBOT_OWNER_ID")
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();
    let secret_token = env::var("TELEGRAM_SECRET_CODE").unwrap_or("".to_string());
    let bot = teloxide::Bot::new(bot_token);
    let telegram_repo = TelegramRepo::new(db_con);
    let telegram = TelegramService::new(bot, telegram_repo);

    // TODO: move this declaration to config.toml
    let mut bot_info = BotInfo::default();

    // To store all ai agent that we have
    let mut ais: HashMap<String, BotAgent> = HashMap::new();

    // for gemini that compatible with openai API
    let gemini_api_key = env::var("GEMINI_API_KEY").unwrap_or("".to_string());
    bot_info.base_url = "https://generativelanguage.googleapis.com/v1beta/openai".to_string();
    bot_info.api_key = gemini_api_key;
    bot_info.model = "gemini-3.1-flash-lite".to_string();
    let gemini_3_1_flash = BotAgent::new(bot_info.clone(), ProviderName::Together, None).await;
    ais.insert("gemini_3.1_flash_google".to_string(), gemini_3_1_flash);

    // for openai gemma 4
    let gemma_open_router_api = env::var("OPENROUTERAPI").unwrap_or("".to_string());
    bot_info.base_url = "https://openrouter.ai/api/v1".to_string();
    bot_info.api_key = gemma_open_router_api;
    bot_info.model = "google/gemma-4-31b-it:free".to_string();
    let gemma_openrouter_4_gb = BotAgent::new(bot_info, ProviderName::OpenRouter, None).await;
    ais.insert("gemma_4_openrouter".to_string(), gemma_openrouter_4_gb);

    let app_state = state::AppState::new(telegram, owner_chat_id, secret_token.into(), ais);
    let backend_api_routes = create_routes(app_state.clone());

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
    Ok(())
}

async fn run_migrations(db: &sea_orm::DatabaseConnection) -> Result<(), SwanError> {
    use migration::{Migrator, MigratorTrait};
    Migrator::up(db, None).await?;
    tracing::info!("Successfully migrate the database ...");

    Ok(())
}
