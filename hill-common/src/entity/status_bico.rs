use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "status_bicos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub retorno: String,
    pub numero: i32,
    pub bloqueado: bool,
    pub status: Option<String>,
    pub quantidade: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type StatusBico = Model;
