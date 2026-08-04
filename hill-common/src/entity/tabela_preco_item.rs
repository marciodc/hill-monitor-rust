use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "tabela_preco_itens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_name = "tabela_preco_id")]
    #[serde(rename = "tabelapreco_id")]
    pub tabela_preco_id: i32,
    pub produto_id: i32,
    pub valor_comercial: Decimal,
    pub valor_tributacao: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type TabelaPrecoItem = Model;
