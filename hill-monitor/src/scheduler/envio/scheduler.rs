use crate::scheduler::envio::{abastecimento, afericao, venda};
use hill_common::config_helper::ConfigHelper;
use hill_common::net::HttpConn;
use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info};

pub struct EnvioScheduler {
    db: DatabaseConnection,
    running: Arc<AtomicBool>,
}

impl EnvioScheduler {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let running = self.running.clone();

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(20)); // Roda a cada 20 segundos
            let config_helper = ConfigHelper::new(db.clone());
            let http = HttpConn::new();

            info!("EnvioScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;

                // Fetch parameters dynamically
                let backend_url = match config_helper.get_parametro("Backend_URL", None).await {
                    Ok(Some(url)) if !url.is_empty() => url,
                    _ => {
                        info!("Backend_URL não configurada. Ignorando envio.");
                        continue;
                    }
                };

                let token = match config_helper.get_parametro("Backend_Token", None).await {
                    Ok(Some(t)) if !t.is_empty() => t,
                    _ => {
                        info!("Backend_Token não configurado. Ignorando envio.");
                        continue;
                    }
                };

                let configuracoes = match config_helper.list_configuracoes().await {
                    Ok(configs) => configs,
                    Err(e) => {
                        error!("Erro ao carregar PDVs para envio: {:?}", e);
                        continue;
                    }
                };

                let Some(configuracao) = configuracoes.first() else {
                    info!("Nenhuma configuração local disponível. Ignorando envio.");
                    continue;
                };

                let empresa_id = configuracao.empresa;
                let tipo_estabelecimento = configuracao.tipo_estabelecimento.clone().unwrap_or_default();

                if tipo_estabelecimento == "posto" || tipo_estabelecimento.is_empty() {
                    // let _ = abastecimento::envia_abastecimentos(
                    //     &db,
                    //     &http,
                    //     &backend_url,
                    //     &token,
                    //     empresa_id,
                    // )
                    // .await;
                    let _ = afericao::envia_afericoes(
                        &db,
                        &http,
                        &backend_url,
                        &token,
                        empresa_id,
                    )
                    .await;
                }

                let _ = venda::envia_vendas(
                    &db,
                    &http,
                    &backend_url,
                    &token,
                    empresa_id,
                    &tipo_estabelecimento,
                )
                .await;
            }
            info!("EnvioScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
