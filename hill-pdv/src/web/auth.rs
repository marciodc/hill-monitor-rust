use crate::web::service::response::ApiResponse;
use axum::extract::Request;
use axum::http::{header, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub const AUTH_TOKEN: &str = "Bearer bf6ae367-6d9b-4ba6-acc7-ddc16008483a";

pub fn is_valid_authorization(header_value: Option<&HeaderValue>) -> bool {
    header_value
        .and_then(|value| value.to_str().ok())
        .map(|value| value == AUTH_TOKEN)
        .unwrap_or(false)
}

pub async fn require_bearer_token(request: Request, next: Next) -> Response {
    if !is_valid_authorization(request.headers().get(header::AUTHORIZATION)) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<()>::err("Token inválido")),
        )
            .into_response();
    }

    next.run(request).await
}
