use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "vendas_pagamentos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub sequencia: i32,
    pub venda_id: Uuid,
    pub forma_pagamento_id: i32,
    pub subtotal: Decimal,
    pub desconto: Decimal,
    pub acrescimo: Decimal,
    pub total: Decimal,
    pub recebido: Decimal,
    pub troco: Decimal,
    pub vinculado: Option<String>,
    pub data_vencimento: Option<DateTime>,
    pub vendedor_id: Option<i32>,
    pub voucher_id: Option<i32>,
    pub tef: Option<String>,
    pub tef_pos: Option<String>,
    pub tef_terminal: Option<String>,
    pub tef_cnpj: Option<String>,
    pub tef_nsu: Option<String>,
    pub tef_rede: Option<String>,
    pub tef_bandeira: Option<String>,
    pub tef_operacao: Option<String>,
    pub tef_parcelas: i32,
    pub tef_tipo_parcelamento: Option<String>,
    pub tef_tipo_transacao: Option<i32>,
    pub tef_desconto: Option<Decimal>,
    pub tef_saque: Option<Decimal>,
    pub tef_sitef_instituicao: Option<String>,
    pub tef_sitef_bandeira: Option<String>,
    pub tef_via_estabelecimento: Option<String>,
    pub tef_via_cliente: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type VendaPagamento = Model;
