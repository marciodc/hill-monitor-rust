use crate::operation::ConcentradorOperacao;
use sea_orm::DatabaseConnection;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::info;

pub struct ConcentradorScheduler {
    op: ConcentradorOperacao,
    db: DatabaseConnection,
    running: Arc<AtomicBool>,
}

impl ConcentradorScheduler {
    pub fn new(op: ConcentradorOperacao, db: DatabaseConnection) -> Self {
        Self {
            op,
            db,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let op = self.op.clone();
        let db = self.db.clone();
        let running = self.running.clone();

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(2));
            let mut tick_count = 0;

            info!("ConcentradorScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;

                // Query status and update
                let status = op.status_bicos(&db).await;
                if let Ok(payload_str) = serde_json::to_string(&status) {
                    let _ = hill_common::event::get_event_bus().send(hill_common::event::AppMessage {
                        tipo: hill_common::event::TipoEvento::EvtStatusAbastecimento,
                        mensagem: payload_str,
                    });
                }

                // Capture pump sales
                op.captura_abastecimento(&db).await;

                tick_count += 1;
                if tick_count >= 20 {
                    tick_count = 0;
                    op.atualiza_preco_banco(&db).await;
                }
            }

            info!("ConcentradorScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
