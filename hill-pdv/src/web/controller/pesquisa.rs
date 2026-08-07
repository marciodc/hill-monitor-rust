use crate::web::service::pesquisa::{PesquisaService, ProdutoPesquisaItem};
use crate::web::service::response::ApiResponse;
use crate::web::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use hill_common::entity::{TabelaPreco, Usuario, Vendedor};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PesquisaUsuarioRequest {
    pub nome: Option<String>,
    pub ids: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PesquisaVendedorRequest {
    pub nome: Option<String>,
    pub codigo: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PesquisaProdutoRequest {
    pub setor_id: i32,
    pub descricao: Option<String>,
    pub pagina: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PesquisaTabelaPrecoRequest {
    pub descricao: Option<String>,
    pub id: Option<i32>,
}

pub async fn pesquisa_usuario(
    State(state): State<AppState>,
    Json(payload): Json<PesquisaUsuarioRequest>,
) -> (StatusCode, Json<ApiResponse<Vec<Usuario>>>) {
    let service = PesquisaService::new(state.db);
    (
        StatusCode::OK,
        Json(service.pesquisa_usuario(payload.nome, payload.ids).await),
    )
}

pub async fn pesquisa_vendedor(
    State(state): State<AppState>,
    Json(payload): Json<PesquisaVendedorRequest>,
) -> (StatusCode, Json<ApiResponse<Vec<Vendedor>>>) {
    let service = PesquisaService::new(state.db);
    (
        StatusCode::OK,
        Json(
            service
                .pesquisa_vendedor(payload.nome, payload.codigo)
                .await,
        ),
    )
}

pub async fn pesquisa_produto(
    State(state): State<AppState>,
    Json(payload): Json<PesquisaProdutoRequest>,
) -> (StatusCode, Json<ApiResponse<Vec<ProdutoPesquisaItem>>>) {
    let service = PesquisaService::new(state.db);
    (
        StatusCode::OK,
        Json(
            service
                .pesquisa_produto(payload.setor_id, payload.descricao, payload.pagina)
                .await,
        ),
    )
}

pub async fn pesquisa_tabela_preco(
    State(state): State<AppState>,
    Json(payload): Json<PesquisaTabelaPrecoRequest>,
) -> (StatusCode, Json<ApiResponse<Vec<TabelaPreco>>>) {
    let service = PesquisaService::new(state.db);
    (
        StatusCode::OK,
        Json(
            service
                .pesquisa_tabela_preco(payload.descricao, payload.id)
                .await,
        ),
    )
}
