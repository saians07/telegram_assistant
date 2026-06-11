use crate::api::{common, v1::telegram};
use crate::dto::response::BaseResponse;
use crate::state::AppState;

use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        common::health,
        telegram::listen,
        telegram::set_new_webhook,
        telegram::remove_webhook,
        telegram::get_webhook_info,
    ),
    components(
        schemas(BaseResponse)
    ),
    tags(
        (name="common", description="Server common endpoints"),
        (name="telegram", description="Telegram Endpoint")
    ),
    info(
        title="Swan API",
        version="1.0.0",
        description="Simple API documentation"
    )
)]
struct ApiDoc;

pub fn create_routes() -> Router<AppState> {
    let common_routes = Router::new().route("/common/health", get(common::health));
    let telegram_routes = Router::new()
        .route("/telegram/listener", post(telegram::listen))
        .route("/telegram/remove_webhook", get(telegram::remove_webhook))
        .route("/telegram/webhook_info", get(telegram::get_webhook_info))
        .route("/telegram/set_webhook", post(telegram::set_new_webhook));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(common_routes)
        .nest("/api/v1", telegram_routes)
}
