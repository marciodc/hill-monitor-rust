use crate::web::service::response::ApiResponse;
use hill_common::config_helper::ConfigHelper;
use hill_common::entity::{PrecoBico, StatusBico};
use hill_concentrador::com::ConcentradorCom;
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
}

impl ConcentradorService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn build_operacao(&self) -> ConcentradorOperacao {
        let config = ConfigHelper::new(self.db.clone());
        let serial_port = config
            .get_parametro("CONCENTRADOR_Porta", None)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "COM1".to_string());

        let com = ConcentradorCom::new(&serial_port);
        ConcentradorOperacao::new(com, "companytec")
    }

    pub async fn status(&self) -> Vec<StatusBico> {
        self.build_operacao().await.status_bicos(&self.db).await
    }

    pub async fn consulta_encerrante(&self, retorno: &str) -> ApiResponse<EncerrantePayload> {
        let encerrante = self
            .build_operacao()
            .await
            .consulta_encerrante(retorno, 2)
            .await;

        ApiResponse::ok(EncerrantePayload { encerrante })
    }

    pub async fn atualiza_preco_bico(&self, preco_bico: PrecoBico) -> ApiResponse<()> {
        let ok = self
            .build_operacao()
            .await
            .atualiza_preco_vista(&preco_bico.retorno, preco_bico.valor_unitario)
            .await;

        if ok {
            ApiResponse::ok_message("OK")
        } else {
            ApiResponse::err("Não foi possível atualizar o preço do bico.")
        }
    }
}
