use crate::web::service::response::ApiResponse;
use crate::web::service::venda_item::{VendaItemPayload, VendaItemService};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::DatabaseConnection;

pub async fn lista_itens(
    State(db): State<DatabaseConnection>,
    Path(venda_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<VendaItemPayload>>>) {
    let service = VendaItemService::new(db);
    let response = service.lista_itens(&venda_id).await;
    let status = if response.status {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, Json(response))
}
