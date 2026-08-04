use crate::web::controller;
use axum::routing::{get, post};
use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(|| async { "Hill.Monitor API in Rust is running!" }))
        // Login endpoints
        .route("/login/autentica", post(controller::login::autentica))
        .route("/login/valida-usuario", post(controller::login::valida_usuario))
        // Abastecimento endpoints
        .route("/abastecimentos", get(controller::abastecimentos::listar_abastecimentos))
        // Share database connection pool via Axum State
        .with_state(db)
        .layer(CorsLayer::permissive())
}
