use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            error: ApiErrorDetail {
                code: code.to_string(),
                message: message.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.error.code.as_str() {
            "INVALID_REQUEST" | "INVALID_ADDRESS" => StatusCode::BAD_REQUEST,
            "INVALID_SIGNATURE" => StatusCode::UNAUTHORIZED,
            "INSUFFICIENT_BALANCE" | "INVALID_NONCE" => StatusCode::UNPROCESSABLE_ENTITY,
            "TRANSACTION_NOT_FOUND" | "ACCOUNT_NOT_FOUND" | "BLOCK_NOT_FOUND" => {
                StatusCode::NOT_FOUND
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;
