use crate::web::controller::login::{LoginResponse, LoginUser};
use sea_orm::DatabaseConnection;

pub struct LoginService {
    _db: DatabaseConnection,
}

impl LoginService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { _db: db }
    }

    pub async fn autentica(&self, user: LoginUser) -> LoginResponse {
        // Placeholder check matching the controller logic
        if user.login == "admin" && user.senha == "admin" {
            LoginResponse {
                status: true,
                mensagem: "Autenticado com sucesso!".to_string(),
            }
        } else {
            LoginResponse {
                status: false,
                mensagem: "Usuário ou senha incorretos.".to_string(),
            }
        }
    }

    pub async fn valida_usuario(&self, user: LoginUser) -> LoginResponse {
        LoginResponse {
            status: true,
            mensagem: format!("Usuário {} é válido.", user.login),
        }
    }
}
