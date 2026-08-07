use crate::web::service::login::LoginService;
use crate::web::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use hill_common::entity::Usuario;
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
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<LoginResponse>) {
    let service = LoginService::new(state.db);
    let response = service.autentica(payload).await;
    if response.status {
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::BAD_REQUEST, Json(response))
    }
}

pub async fn valida_usuario(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<LoginResponse>) {
    let service = LoginService::new(state.db);
    let response = service.valida_usuario(payload).await;
    if response.status {
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::BAD_REQUEST, Json(response))
    }
}
