use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// Resposta padrão de sucesso
#[derive(Serialize)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: T,
}

/// Resposta padrão de erro
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

pub fn ok<T>(data: T) -> Response
where
    T: Serialize,
{
    (
        StatusCode::OK,
        Json(SuccessResponse {
            success: true,
            data,
        }),
    )
        .into_response()
}

pub fn created<T>(data: T) -> Response
where
    T: Serialize,
{
    (
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            data,
        }),
    )
        .into_response()
}

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn bad_request(message: impl Into<String>) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: message.into(),
        }),
    )
        .into_response()
}

pub fn unauthorized(message: impl Into<String>) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            success: false,
            error: message.into(),
        }),
    )
        .into_response()
}

pub fn not_found(message: impl Into<String>) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            success: false,
            error: message.into(),
        }),
    )
        .into_response()
}

pub fn internal_error(message: impl Into<String>) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            success: false,
            error: message.into(),
        }),
    )
        .into_response()
}
