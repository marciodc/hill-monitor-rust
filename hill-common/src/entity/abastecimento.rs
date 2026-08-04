use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "abastecimentos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub pdv: Option<Uuid>,
    pub status: Option<String>,
    pub bloqueado: Option<String>,
    pub bico_id: i32,
    pub retorno: Option<String>,
    pub quantidade: Decimal,
    pub valor_unitario: Decimal,
    pub total: Decimal,
    pub tempo: Option<String>,
    pub data_hora: DateTime,
    pub encerrante_inicial: Decimal,
    pub encerrante_final: Decimal,
    pub rfid_frentista: Option<String>,
    pub rfid_cliente: Option<String>,
    pub gerado: Option<String>,
    pub desmembramento_id: Option<Uuid>,
    pub full_string: Option<String>,
    pub sincronizado: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Abastecimento = Model;
