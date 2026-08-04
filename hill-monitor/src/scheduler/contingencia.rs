use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::info;
use uuid::Uuid;

pub struct ContingenciaScheduler {
    db: DatabaseConnection,
    pdv_uuid: Uuid,
    running: Arc<AtomicBool>,
}

impl ContingenciaScheduler {
    pub fn new(db: DatabaseConnection, pdv_uuid: Uuid) -> Self {
        Self {
            db,
            pdv_uuid,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let _db = self.db.clone();
        let running = self.running.clone();
        let pdv_uuid = self.pdv_uuid;

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Roda a cada 30 segundos
            info!("ContingenciaScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;
                info!("ContingenciaScheduler processando contingência para o PDV: {}", pdv_uuid);
                
                // TODO: Chamar o processo de envio de NFCe em contingência
            }
            info!("ContingenciaScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
