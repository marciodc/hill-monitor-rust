use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "produtos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub tipo: Option<String>,
    pub categoria: Option<String>,
    pub descricao: String,
    pub descricao_resumida: Option<String>,
    pub gtin_tributacao: Option<String>,
    pub gtin_comercial: Option<String>,
    pub unidade_comercial: Option<String>,
    pub unidade_tributacao: Option<String>,
    pub quantidade_tributacao: Decimal,
    pub ncm: Option<String>,
    pub ncm_excecao: Option<String>,
    pub imposto_aliquota_importacao: Decimal,
    pub imposto_aliquota_federal: Decimal,
    pub imposto_aliquota_estadual: Decimal,
    pub imposto_aliquota_municipal: Decimal,
    pub imposto_chave: Option<String>,
    pub tipo_codigo: i32,
    pub codigo: Option<String>,
    pub codigo_auxiliar: Option<String>,
    pub indicador_producao: Option<String>,
    pub fracionado: Option<String>,
    pub pesado_caixa: Option<String>,
    pub cst: Option<String>,
    pub cst_pis: Option<String>,
    pub cst_cofins: Option<String>,
    pub observacao: Option<String>,
    pub codigo_anp: i32,
    pub descricao_anp: Option<String>,
    pub solicita_vendedor: Option<String>,
    pub grade_id: Option<i32>,
    pub controla_numero_serie: Option<String>,
    pub controla_lote: Option<String>,
    pub setor_impressao_1: Option<i32>,
    pub setor_impressao_2: Option<i32>,
    pub setor_impressao_3: Option<i32>,
    pub setor_impressao_4: Option<i32>,
    #[serde(rename = "exclusito_kit")]
    pub exclusivo_kit: Option<String>,
    pub cest: Option<String>,
    pub cfop: i32,
    pub aliquota: Decimal,
    pub aliquota_cofins: Decimal,
    pub aliquota_pis: Decimal,
    pub tipo_combustivel: i32,
    pub etiqueta_balanca: Option<String>,
    pub predbcefet: Option<Decimal>,
    pub picmsefet: Option<Decimal>,
    pub pfcpstret: Option<Decimal>,
    pub pfcpst: Option<Decimal>,
    pub pfcp: Option<Decimal>,
    pub modbc: Option<Decimal>,
    pub modbcst: Option<Decimal>,
    pub pmvast: Option<Decimal>,
    pub predbcst: Option<Decimal>,
    pub picmsst: Option<Decimal>,
    pub predbc: Option<Decimal>,
    pub pglp: Option<Decimal>,
    pub pgnn: Option<Decimal>,
    pub pgni: Option<Decimal>,
    pub vpart: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Produto = Model;
