use crate::web::service::response::ApiResponse;
use hill_common::entity::{PrecoBico, StatusBico};
use hill_concentrador::operation::ConcentradorOperacao;
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
pub struct EncerrantePayload {
    #[serde(rename = "Encerrante")]
    pub encerrante: Decimal,
}

pub struct ConcentradorService {
    db: DatabaseConnection,
    op: ConcentradorOperacao,
}

impl ConcentradorService {
    pub fn new(db: DatabaseConnection, op: ConcentradorOperacao) -> Self {
        Self { db, op }
    }

    pub async fn status(&self) -> Vec<StatusBico> {
        self.op.status_bicos(&self.db).await
    }

    pub async fn consulta_encerrante(&self, retorno: &str) -> ApiResponse<EncerrantePayload> {
        let encerrante = self.op.consulta_encerrante(retorno, 2).await;
        ApiResponse::ok(EncerrantePayload { encerrante })
    }

    pub async fn atualiza_preco_bico(&self, preco_bico: PrecoBico) -> ApiResponse<()> {
        let ok = self
            .op
            .atualiza_preco_vista(&preco_bico.retorno, preco_bico.valor_unitario)
            .await;

        if ok {
            ApiResponse::ok_message("OK")
        } else {
            ApiResponse::err("Não foi possível atualizar o preço do bico.")
        }
    }
}
