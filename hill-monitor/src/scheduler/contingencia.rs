use hill_common::config_helper::ConfigHelper;
use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info};

pub struct ContingenciaScheduler {
    db: DatabaseConnection,
    running: Arc<AtomicBool>,
}

impl ContingenciaScheduler {
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
            let mut interval = interval(Duration::from_secs(30)); // Roda a cada 30 segundos
            let config_helper = ConfigHelper::new(db);
            info!("ContingenciaScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;

                let configuracoes = match config_helper.list_configuracoes().await {
                    Ok(configs) => configs,
                    Err(e) => {
                        error!("Erro ao carregar PDVs para contingência: {:?}", e);
                        continue;
                    }
                };

                for configuracao in configuracoes {
                    info!(
                        "ContingenciaScheduler processando contingência para o PDV: {:?}",
                        configuracao.id
                    );
                }

                // TODO: Chamar o processo de envio de NFCe em contingência
            }
            info!("ContingenciaScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
