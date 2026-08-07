use crate::web::service::concentrador::{ConcentradorService, EncerrantePayload};
use crate::web::service::response::ApiResponse;
use crate::web::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use hill_common::entity::{PrecoBico, StatusBico};

pub async fn status(State(state): State<AppState>) -> (StatusCode, Json<Vec<StatusBico>>) {
    let service = ConcentradorService::new(state.db, state.concentrador_op);
    (StatusCode::OK, Json(service.status().await))
}

pub async fn consulta_encerrante(
    State(state): State<AppState>,
    Path(retorno): Path<String>,
) -> (StatusCode, Json<ApiResponse<EncerrantePayload>>) {
    let service = ConcentradorService::new(state.db, state.concentrador_op);
    (
        StatusCode::OK,
        Json(service.consulta_encerrante(&retorno).await),
    )
}

pub async fn atualiza_preco(
    State(state): State<AppState>,
    Json(payload): Json<PrecoBico>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    let service = ConcentradorService::new(state.db, state.concentrador_op);
    let response = service.atualiza_preco_bico(payload).await;
    let status = if response.status {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, Json(response))
}
