use crate::scheduler::atualizacao::AtualizacaoScheduler;
use crate::scheduler::contingencia::ContingenciaScheduler;
use crate::scheduler::envio::EnvioScheduler;
use sea_orm::DatabaseConnection;

pub struct MonitorSchedulers {
    atualizacao: AtualizacaoScheduler,
    contingencia: ContingenciaScheduler,
    envio: EnvioScheduler,
}

impl MonitorSchedulers {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            atualizacao: AtualizacaoScheduler::new(db.clone()),
            contingencia: ContingenciaScheduler::new(db.clone()),
            envio: EnvioScheduler::new(db),
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
