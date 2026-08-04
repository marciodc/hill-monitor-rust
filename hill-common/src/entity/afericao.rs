use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "afericoes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub pdv: Uuid,
    pub setor_id: i32,
    pub turno_posto_id: Uuid,
    pub data_hora: DateTime,
    pub abastecimento_id: Uuid,
    pub bico_id: i32,
    pub quantidade: Decimal,
    pub usuario_id: i32,
    pub sincronizado: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Afericao = Model;
