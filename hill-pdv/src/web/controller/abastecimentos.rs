use crate::web::service::abastecimentos::AbastecimentoService;
use crate::web::service::response::ApiResponse;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use hill_common::entity::Abastecimento;
use sea_orm::DatabaseConnection;

pub async fn listar_abastecimentos(
    State(db): State<DatabaseConnection>,
) -> (StatusCode, Json<Vec<Abastecimento>>) {
    let service = AbastecimentoService::new(db);
    match service.listar_abastecimentos().await {
        Ok(abast) => (StatusCode::OK, Json(abast)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

pub async fn listar_abastecimentos_usuario(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Vec<Abastecimento>>) {
    let service = AbastecimentoService::new(db);
    match service.listar_abastecimentos_usuario(&id).await {
        Ok(abast) => (StatusCode::OK, Json(abast)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

pub async fn localizar_abastecimento(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Abastecimento>>) {
    let service = AbastecimentoService::new(db);
    match service.localizar_abastecimento(&id).await {
        Ok(Some(abast)) => (StatusCode::OK, Json(ApiResponse::ok(abast))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ApiResponse::err("Dados não localizados"))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::err("Erro ao buscar abastecimento.")),
        ),
    }
}

pub async fn seleciona_abastecimento(
    State(db): State<DatabaseConnection>,
    Path((pdv, id)): Path<(String, String)>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    let service = AbastecimentoService::new(db);
    let response = service.seleciona_abastecimento(&pdv, &id).await;
    let status = if response.status {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, Json(response))
}

pub async fn desseleciona_abastecimento(
    State(db): State<DatabaseConnection>,
    Path((pdv, id)): Path<(String, String)>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    let service = AbastecimentoService::new(db);
    let response = service.desseleciona_abastecimento(&pdv, &id).await;
    let status = if response.status {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, Json(response))
}
