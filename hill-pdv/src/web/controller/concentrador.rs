use crate::web::service::concentrador::{ConcentradorService, EncerrantePayload};
use crate::web::service::response::ApiResponse;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use hill_common::entity::{PrecoBico, StatusBico};
use sea_orm::DatabaseConnection;

pub async fn status(
    State(db): State<DatabaseConnection>,
) -> (StatusCode, Json<Vec<StatusBico>>) {
    let service = ConcentradorService::new(db);
    (StatusCode::OK, Json(service.status().await))
}

pub async fn consulta_encerrante(
    State(db): State<DatabaseConnection>,
    Path(retorno): Path<String>,
) -> (StatusCode, Json<ApiResponse<EncerrantePayload>>) {
    let service = ConcentradorService::new(db);
    (StatusCode::OK, Json(service.consulta_encerrante(&retorno).await))
}

pub async fn atualiza_preco(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<PrecoBico>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    let service = ConcentradorService::new(db);
    let response = service.atualiza_preco_bico(payload).await;
    let status = if response.status {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, Json(response))
}
