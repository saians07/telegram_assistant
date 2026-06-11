use anyhow::Result;
use axum::{
    extract::{FromRequestParts, Request},
    http::HeaderValue,
    response::Response,
};
use axum_client_ip::ClientIp;
use futures_util::future::BoxFuture;
use tokio::time::Instant;
use tower::{Layer, Service};
use tracing::Instrument;
use uuid::Uuid;

pub struct RequestLogLayer;

#[derive(Debug, Clone, Default)]
pub struct RequestLayer;

#[derive(Debug, Clone)]
pub struct RequestId {
    id: String,
    ip_address: String,
    user_agent: String,
}

const SWAN_REQ_ID: &str = "X-Swan-Request-ID";

#[derive(Debug, Clone)]
pub struct RequestService<S> {
    inner: S,
}

impl<S> Layer<S> for RequestLayer {
    type Service = RequestService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestService { inner }
    }
}

impl<S> Service<Request> for RequestService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let (mut parts, body) = req.into_parts();

        let start = Instant::now();
        let method = parts.method.to_owned();
        let uri = parts.uri.to_owned();
        let version = parts.version.to_owned();
        let request_id = parts
            .headers
            .get(SWAN_REQ_ID)
            .and_then(|v| v.to_str().ok())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| Uuid::new_v4().to_string())
            .to_owned();
        let user_agent = parts
            .headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned())
            .unwrap_or("unknown".to_string())
            .to_owned();

        let device_type = if user_agent.contains("Mobile")
            || user_agent.contains("Android") && !user_agent.contains("Tables")
        {
            "Mobile"
        } else if user_agent.contains("Tablet") || user_agent.contains("iPad") {
            "Tablet"
        } else {
            "PC"
        };

        let os = if user_agent.contains("Windows") {
            "Windows"
        } else if user_agent.contains("Macintosh") || user_agent.contains("Mac OS") {
            "MacOS"
        } else if user_agent.contains("Linux") {
            "Linux"
        } else if user_agent.contains("Android") {
            "Android"
        } else if user_agent.contains("iPhone") || user_agent.contains("iPad") {
            "iOS"
        } else {
            "Unknown"
        };

        if !parts.headers.contains_key(SWAN_REQ_ID) {
            if let Ok(header_value) = HeaderValue::from_str(&request_id) {
                parts.headers.insert(SWAN_REQ_ID, header_value);
            }
        }

        let span = tracing::info_span!(
            "Request",
            method = %method,
            uri = %uri,
            version = ?version,
            request_id = %request_id,
            user_agent = %user_agent,
            ip_address = tracing::field::Empty,
            status = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
        );

        let mut inner = self.inner.to_owned();

        Box::pin(
            async move {
                tracing::info!("Incoming Request ...");
                let mut id_request = RequestId::new(
                    request_id.to_owned(),
                    "unknown".to_string(),
                    user_agent.to_owned(),
                );
                match ClientIp::from_request_parts(&mut parts, &()).await {
                    Ok(ip) => {
                        id_request.ip_address = ip.0.to_string().to_owned();
                        tracing::Span::current()
                            .record("ip_address", &id_request.ip_address.to_owned());
                        parts.extensions.insert(id_request.clone());
                        parts.extensions.insert(RequestInfo {
                            device_name: Some(os.to_string()),
                            device_type: Some(device_type.to_string()),
                            ip_address: Some(ip.0.to_string().to_owned()),
                        });
                    }
                    _ => {
                        tracing::Span::current().record("ip_address", "unknown");
                        parts.extensions.insert(id_request.clone());
                        parts.extensions.insert(RequestInfo {
                            device_name: Some(os.to_string()),
                            device_type: Some(device_type.to_string()),
                            ip_address: None,
                        });
                    }
                }

                let req = Request::from_parts(parts, body);
                tracing::Span::current().record("User Agent", id_request.user_agent);

                let response = inner.call(req).await?;
                let duration = start.elapsed();
                let status = response.status();
                let status_code = status.as_u16();

                tracing::Span::current().record("Status", status_code);
                tracing::Span::current().record("Duration [ms]", duration.as_millis());

                match status_code {
                    500..=599 => {
                        tracing::error!("Internal Server Error!");
                    }
                    400..=499 => {
                        tracing::warn!("Client Error")
                    }
                    200..=299 => {
                        tracing::info!("HTTP OK!")
                    }
                    _ => {
                        tracing::info!("Completed Successfully!")
                    }
                }

                Ok(response)
            }
            .instrument(span),
        )
    }
}

impl RequestId {
    fn new(id: String, ip_address: String, user_agent: String) -> Self {
        Self {
            id,
            ip_address,
            user_agent,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub ip_address: Option<String>,
}
