use axum::Json;
use serde_json::{Value, json};

#[
    utoipa::path(
        get,
        path="/common/health",
        tag="common",
        responses(
            (status=200, description="{Server is healthy!}")
        )
    )
]
pub async fn health() -> Json<Value> {
    Json(json!({
        "status": 200,
        "message": "Server is healthy!"
    }))
}
