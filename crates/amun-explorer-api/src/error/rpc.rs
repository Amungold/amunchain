use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt;

#[derive(Debug)]
pub enum RpcError {
    Network(reqwest::Error),
    Json(serde_json::Error),
    Rpc {
        http_status: StatusCode,
        code: Option<i64>,
        message: String,
    },
}

// ------------------------------------------------------------------
// Constructors (for old service code compatibility — TEMPORARY)
// ------------------------------------------------------------------
impl RpcError {
    pub fn new(code: &str, message: &str) -> Self {
        Self::Rpc {
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
            code: Some(code.parse().unwrap_or(500)),
            message: message.to_string(),
        }
    }

    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::Rpc {
            http_status: StatusCode::NOT_FOUND,
            code: Some(404),
            message: format!("{} not found: {}", resource, id),
        }
    }
}

// ------------------------------------------------------------------
// IntoResponse — allows handlers to return Result<T, RpcError> directly
// ------------------------------------------------------------------
impl IntoResponse for RpcError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            Self::Network(e) => (
                StatusCode::BAD_GATEWAY,
                format!("{{\"error\": \"network: {e}\"}}"),
            ),
            Self::Json(e) => (
                StatusCode::BAD_GATEWAY,
                format!("{{\"error\": \"json: {e}\"}}"),
            ),
            Self::Rpc {
                http_status,
                message,
                ..
            } => (*http_status, format!("{{\"error\": \"{message}\"}}")),
        };

        (
            status,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response()
    }
}

// ------------------------------------------------------------------
// Display + Error
// ------------------------------------------------------------------
impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(e) => write!(f, "network error: {e}"),
            Self::Json(e) => write!(f, "json error: {e}"),
            Self::Rpc {
                http_status,
                code,
                message,
            } => {
                let status_str = http_status.as_str();
                if let Some(c) = code {
                    write!(f, "rpc error (http={status_str}, code={c}): {message}")
                } else {
                    write!(f, "rpc error (http={status_str}): {message}")
                }
            }
        }
    }
}

impl std::error::Error for RpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(e) => Some(e),
            Self::Json(e) => Some(e),
            Self::Rpc { .. } => None,
        }
    }
}

impl From<reqwest::Error> for RpcError {
    fn from(e: reqwest::Error) -> Self {
        Self::Network(e)
    }
}

impl From<serde_json::Error> for RpcError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
