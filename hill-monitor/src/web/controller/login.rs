use crate::web::service::login::LoginService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;

#[derive(Deserialize)]
pub struct LoginUser {
    pub login: String,
    pub senha: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: bool,
    pub mensagem: String,
}

pub async fn autentica(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<LoginResponse>) {
    let service = LoginService::new(db);
    let response = service.autentica(payload).await;
    if response.status {
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::BAD_REQUEST, Json(response))
    }
}

pub async fn valida_usuario(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<LoginResponse>) {
    let service = LoginService::new(db);
    let response = service.valida_usuario(payload).await;
    (StatusCode::OK, Json(response))
}
