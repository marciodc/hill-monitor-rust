use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "bicos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub status: Option<String>,
    pub retorno: Option<String>,
    pub numero: i32,
    pub bomba: i32,
    pub tanque_id: i32,
    pub bloqueio_quantidade: i32,
    pub setor_id: i32,
    pub produto_id: i32,
    pub gtin: Option<String>,
    pub valor_unitario: Decimal,
    pub valor_unitario_debito: Decimal,
    pub valor_unitario_credito: Decimal,
    pub combustivel: Option<String>,
    pub altera_preco: Option<String>,
    pub tabelapreco_id: i32,
    pub tipo_combustivel: Option<String>,
    pub abastecimento_manual: Option<String>,
    pub bloqueado: Option<String>,
    pub sincroniza_preco_data_hora: Option<DateTime>,
    pub sincroniza_preco_alterado: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Bico = Model;
