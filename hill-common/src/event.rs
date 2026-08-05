use std::sync::OnceLock;
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub enum TipoEvento {
    EvtStatusAbastecimento,
}

#[derive(Clone, Debug)]
pub struct AppMessage {
    pub tipo: TipoEvento,
    pub mensagem: String,
}

static EVENT_BUS: OnceLock<broadcast::Sender<AppMessage>> = OnceLock::new();

pub fn get_event_bus() -> &'static broadcast::Sender<AppMessage> {
    EVENT_BUS.get_or_init(|| {
        let (tx, _rx) = broadcast::channel(100);
        tx
    })
}
