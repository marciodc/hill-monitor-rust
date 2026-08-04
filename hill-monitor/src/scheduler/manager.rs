use crate::scheduler::atualizacao::AtualizacaoScheduler;
use crate::scheduler::contingencia::ContingenciaScheduler;
use crate::scheduler::envio::EnvioScheduler;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct MonitorSchedulers {
    atualizacao: AtualizacaoScheduler,
    contingencia: ContingenciaScheduler,
    envio: EnvioScheduler,
}

impl MonitorSchedulers {
    pub fn new(db: DatabaseConnection, pdv_uuid: Uuid) -> Self {
        Self {
            atualizacao: AtualizacaoScheduler::new(db.clone(), pdv_uuid),
            contingencia: ContingenciaScheduler::new(db.clone(), pdv_uuid),
            envio: EnvioScheduler::new(db, pdv_uuid),
        }
    }

    pub fn start(&self) {
        self.atualizacao.start();
        self.contingencia.start();
        self.envio.start();
    }

    pub fn stop(&self) {
        self.atualizacao.stop();
        self.contingencia.stop();
        self.envio.stop();
    }
}
