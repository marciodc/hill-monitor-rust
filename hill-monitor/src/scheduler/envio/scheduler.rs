use crate::scheduler::envio::{abastecimento, afericao, venda};
use hill_common::config_helper::ConfigHelper;
use hill_common::net::HttpConn;
use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::info;
use uuid::Uuid;

pub struct EnvioScheduler {
    db: DatabaseConnection,
    pdv_uuid: Uuid,
    running: Arc<AtomicBool>,
}

impl EnvioScheduler {
    pub fn new(db: DatabaseConnection, pdv_uuid: Uuid) -> Self {
        Self {
            db,
            pdv_uuid,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let running = self.running.clone();
        let pdv_uuid = self.pdv_uuid;

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(20)); // Roda a cada 20 segundos
            let config_helper = ConfigHelper::new(db.clone());
            let http = HttpConn::new();

            info!("EnvioScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;

                // Fetch parameters dynamically
                let backend_url = match config_helper.get_parametro("PDV_BackendURL", Some(pdv_uuid)).await {
                    Ok(Some(url)) if !url.is_empty() => url,
                    _ => {
                        info!("PDV_BackendURL não configurada. Ignorando envio.");
                        continue;
                    }
                };

                let token = match config_helper.get_parametro("PDV_Token", Some(pdv_uuid)).await {
                    Ok(Some(t)) if !t.is_empty() => t,
                    _ => {
                        info!("PDV_Token não configurado. Ignorando envio.");
                        continue;
                    }
                };

                // Fetch PDV config for Empresa and TipoEstabelecimento
                let (empresa_id, tipo_estabelecimento) = match config_helper.get_config_by_pdv(pdv_uuid).await {
                    Ok(Some(conf)) => (conf.empresa, conf.tipo_estabelecimento.unwrap_or_default()),
                    _ => (0, String::new()),
                };

                // 1. Envio de Abastecimentos e Aferições (somente se for Posto)
                if tipo_estabelecimento == "posto" || tipo_estabelecimento.is_empty() {
                    let _ = abastecimento::envia_abastecimentos(&db, &http, &backend_url, &token, empresa_id).await;
                    let _ = afericao::envia_afericoes(&db, &http, &backend_url, &token, empresa_id).await;
                }

                // 2. Envio de Vendas
                let _ = venda::envia_vendas(&db, pdv_uuid, &http, &backend_url, &token, empresa_id, &tipo_estabelecimento).await;
            }
            info!("EnvioScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
