use hill_concentrador::operation::ConcentradorOperacao;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub concentrador_op: ConcentradorOperacao,
}
