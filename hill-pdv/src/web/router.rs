use crate::web::state::AppState;
use crate::web::{auth, controller};
use axum::Router;
use axum::middleware;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Hill.Monitor API in Rust is running!" }),
        )
        .route(
            "/abastecimentos",
            get(controller::abastecimentos::listar_abastecimentos),
        )
        .route(
            "/abastecimento",
            get(controller::abastecimentos::listar_abastecimentos),
        )
        .route(
            "/abastecimento/{id}/user",
            get(controller::abastecimentos::listar_abastecimentos_usuario),
        )
        .route(
            "/abastecimento/{id}",
            get(controller::abastecimentos::localiza_abastecimento),
        )
        .route(
            "/abastecimento/{pdv}/{id}/seleciona",
            post(controller::abastecimentos::seleciona_abastecimento),
        )
        .route(
            "/abastecimento/{pdv}/{id}/desseleciona",
            post(controller::abastecimentos::desseleciona_abastecimento),
        )
        .route(
            "/concentrador/status",
            get(controller::concentrador::status),
        )
        .route(
            "/concentrador/encerrante/{retorno}",
            get(controller::concentrador::consulta_encerrante),
        )
        .route(
            "/concentrador/atualiza_preco",
            post(controller::concentrador::atualiza_preco),
        )
        .route("/login/autentica", post(controller::login::autentica))
        .route(
            "/login/valida-usuario",
            post(controller::login::valida_usuario),
        )
        .route(
            "/pesquisa/usuario",
            post(controller::pesquisa::pesquisa_usuario),
        )
        .route(
            "/pesquisa/vendedor",
            post(controller::pesquisa::pesquisa_vendedor),
        )
        .route(
            "/pesquisa/produto",
            post(controller::pesquisa::pesquisa_produto),
        )
        .route(
            "/pesquisa/tabela_preco",
            post(controller::pesquisa::pesquisa_tabela_preco),
        )
        .route(
            "/venda-item/venda/{venda_id}",
            get(controller::venda_item::lista_itens),
        )
        .route(
            "/nfce/enviar_nfe/{numero_nota}",
            get(controller::nfce::enviar_nfe),
        )
        .route("/ws", get(super::websocket::ws_handler))
        .layer(middleware::from_fn(auth::require_bearer_token))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
