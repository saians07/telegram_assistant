use axum::http::StatusCode;

#[derive(serde::Serialize, utoipa::ToSchema, Debug)]
pub struct BaseResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<()>,
}

impl BaseResponse {
    pub fn reply(stat: StatusCode, message: &str) -> Self {
        Self {
            status: stat.as_u16(),
            message: message.into(),
            data: None,
        }
    }
}
