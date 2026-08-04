use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "usuarios")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub status: String,
    pub nome: String,
    pub login: String,
    pub senha: String,
    pub rfid: Option<String>,
    pub rfid_debito: Option<String>,
    pub rfid_credito: Option<String>,
    pub digital: Option<String>,
    pub cartao_magnetico: Option<String>,
    pub perc_max_desc_acres_item: Decimal,
    pub valor_max_desc_acres_item: Decimal,
    pub perc_max_desc_acres_subtotal: Decimal,
    pub valor_max_desc_acres_subtotal: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Usuario = Model;
