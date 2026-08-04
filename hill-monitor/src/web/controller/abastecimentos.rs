use crate::web::service::abastecimentos::AbastecimentoService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use hill_common::entity::Abastecimento;
use sea_orm::DatabaseConnection;
use tracing::error;

pub async fn listar_abastecimentos(
    State(db): State<DatabaseConnection>,
) -> (StatusCode, Json<Vec<Abastecimento>>) {
    let service = AbastecimentoService::new(db);
    match service.listar_abastecimentos().await {
        Ok(abast) => (StatusCode::OK, Json(abast)),
        Err(e) => {
            error!("Erro ao buscar abastecimentos do banco: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        }
    }
}
