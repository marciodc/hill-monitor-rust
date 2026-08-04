use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "parceiros")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub status: Option<String>,
    pub cpf_cnpj: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub ie_situacao: Option<String>,
    pub inscricao_municipal: Option<String>,
    pub nome_fantasia: Option<String>,
    pub razao_social: Option<String>,
    #[sea_orm(column_name = "endereco")]
    #[serde(rename = "endereco")]
    pub logradouro: Option<String>,
    pub complemento: Option<String>,
    #[sea_orm(column_name = "endereco_numero")]
    #[serde(rename = "endereco_numero")]
    pub numero: Option<String>,
    pub bairro: Option<String>,
    pub municipio: Option<String>,
    pub cod_municipio: i32,
    pub uf: Option<String>,
    pub cep: Option<String>,
    pub requer_placa: Option<String>,
    pub requer_km: Option<String>,
    pub requer_condutor: Option<String>,
    #[sea_orm(column_name = "desconto_cupom")]
    #[serde(rename = "desconto_cupom")]
    pub desconto_venda: Decimal,
    pub limite_disponivel: Decimal,
    pub email: Option<String>,
    pub rfid: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Parceiro = Model;
