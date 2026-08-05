use crate::web::service::login::LoginService;
use hill_common::entity::Usuario;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub login: String,
    pub senha: String,
    pub pdv: Option<Uuid>,
    pub acao: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub status: bool,
    pub mensagem: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Usuario>,
}

impl LoginResponse {
    pub fn ok(mensagem: impl Into<String>, data: Option<Usuario>) -> Self {
        Self {
            status: true,
            mensagem: mensagem.into(),
            data,
        }
    }

    pub fn err(mensagem: impl Into<String>) -> Self {
        Self {
            status: false,
            mensagem: mensagem.into(),
            data: None,
        }
    }
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
    if response.status {
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::BAD_REQUEST, Json(response))
    }
}
