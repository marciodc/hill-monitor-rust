use crate::web::service::response::ApiResponse;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;

pub async fn enviar_nfe(
    Path(numero_nota): Path<i32>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::err(format!(
            "Endpoint /nfce/enviar_nfe/{} ainda não foi portado para Rust.",
            numero_nota
        ))),
    )
}
