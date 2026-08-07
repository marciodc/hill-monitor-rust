use crate::web::service::response::ApiResponse;
use crate::web::service::venda_item::{VendaItemPayload, VendaItemService};
use crate::web::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub async fn lista_itens(
    State(state): State<AppState>,
    Path(venda_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Vec<VendaItemPayload>>>) {
    let service = VendaItemService::new(state.db);
    let response = service.lista_itens(&venda_id).await;
    let status = if response.status {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, Json(response))
}
