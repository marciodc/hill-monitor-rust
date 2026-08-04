use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "vendas_itens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub status: Option<String>,
    pub venda_id: Uuid,
    pub pre_venda: Option<String>,
    pub sequencia: i32,
    pub produto_id: i32,
    pub produto_gtin: Option<String>,
    pub descricao: Option<String>,
    pub quantidade: Decimal,
    pub valor_comercial: Decimal,
    pub valor_tributacao: Decimal,
    pub subtotal: Decimal,
    pub desconto: Decimal,
    pub acrescimo: Decimal,
    pub total: Decimal,
    pub desconto_fechamento: Decimal,
    pub acrescimo_fechamento: Decimal,
    pub total_fechamento: Decimal,
    pub cfop: i32,
    pub cst: Option<String>,
    pub icms_aliquota: Decimal,
    pub icms_valor: Decimal,
    pub abastecimento_id: Option<Uuid>,
    pub bico_id: Decimal,
    pub encerrante_inicial: Decimal,
    pub encerrante_final: Decimal,
    pub rfid_vendedor: Option<String>,
    pub rfid_cliente: Option<String>,
    pub setor_id: Decimal,
    pub grade_item_id: Decimal,
    pub grade_codigo: Option<String>,
    pub grade_descricao: Option<String>,
    pub lote_id: Decimal,
    pub produto_serie_id: i32,
    pub tabela_preco_id: i32,
    pub tabela_padrao: Option<String>,
    pub vendedor_id: Decimal,
    pub icmsst_valor: Decimal,
    pub icmsst_aliquota: Decimal,
    pub total_tributos: Decimal,
    pub total_tributos_importacao: Decimal,
    pub total_tributos_federal: Decimal,
    pub total_tributos_estadual: Decimal,
    pub total_tributos_municipal: Decimal,
    pub cst_pis: Option<String>,
    pub pis_valor: Decimal,
    pub pis_aliquota: Decimal,
    pub cst_cofins: Option<String>,
    pub cofins_valor: Decimal,
    pub cofins_aliquota: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type VendaItem = Model;
