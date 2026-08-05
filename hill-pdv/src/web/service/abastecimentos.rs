use hill_common::entity::{abastecimento, Abastecimento};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect, DbErr};

pub struct AbastecimentoService {
    db: DatabaseConnection,
}

impl AbastecimentoService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn listar_abastecimentos(&self) -> Result<Vec<Abastecimento>, DbErr> {
        abastecimento::Entity::find()
            .order_by_desc(abastecimento::Column::DataHora)
            .limit(100)
            .all(&self.db)
            .await
    }
}
