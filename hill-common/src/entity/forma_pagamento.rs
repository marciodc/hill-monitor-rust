use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "formas_pagamento")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub numero: i32,
    #[sea_orm(column_name = "forma_pagamento")]
    #[serde(rename = "forma_pagamento")]
    pub tipo_pagamento: i32,
    pub descricao: String,
    pub valor_aviso_sangria: Decimal,
    pub somente_cadastrados: Option<String>,
    pub permite_troco: Option<String>,
    pub permite_desconto: Option<String>,
    pub permite_acrescimo: Option<String>,
    pub dados_cheque: Option<String>,
    pub dados_tef: Option<String>,
    pub maximo_parcelas: i32,
    pub tef_rede: Option<String>,
    pub tef_operacao: i32,
    #[sea_orm(column_name = "ativo_voucher")]
    #[serde(rename = "ativo_voucher")]
    pub voucher: Option<String>,
    pub ignora_limite_troco: Option<String>,
    pub solicita_vencimento: Option<String>,
    pub valida_limite_credito: Option<String>,
    pub espelho: Option<String>,
    pub dias_vencimento: Option<String>,
    #[sea_orm(column_name = "tipo")]
    #[serde(rename = "tipo")]
    pub tipo_venda: Option<String>,
    pub tabela_id: i32,
    pub permite_cheque_troco: Option<String>,
    pub permite_deposito_troco: Option<String>,
    pub percentual_maximo_troco: Decimal,
    pub percentual_desconto: Decimal,
    pub percentual_maximo_desconto: Decimal,
    pub venda_mobile: Option<String>,
    pub troco_em_deposito: Option<String>,
    pub vendas_com_juros_mobile: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type FormaPagamento = Model;
