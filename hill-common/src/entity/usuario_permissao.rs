use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "usuario_permissoes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub usuario_id: i32,
    #[sea_orm(column_name = "cancela_venda_aberta")]
    #[serde(rename = "cancela_cupom_aberto")]
    pub cancela_venda_aberta: Option<String>,
    #[sea_orm(column_name = "cancela_venda_fechada")]
    #[serde(rename = "cancela_cupom_fechado")]
    pub cancela_venda_fechada: Option<String>,
    pub cancela_item: Option<String>,
    pub desconto_item: Option<String>,
    pub desconto_fechamento: Option<String>,
    pub desconto_fechamento_pv: Option<String>,
    pub acrescimo_fechamento: Option<String>,
    pub acrescimo_item: Option<String>,
    pub acrescimo_fechamento_pv: Option<String>,
    pub cliente_limite: Option<String>,
    pub cliente_bloqueado: Option<String>,
    pub cliente_forma_pagamento: Option<String>,
    pub sangria: Option<String>,
    pub suprimento: Option<String>,
    pub abertura_turno: Option<String>,
    pub fechamento_turno: Option<String>,
    pub reabertura_turno: Option<String>,
    pub afericao: Option<String>,
    pub lista_todos_abastecimentos: Option<String>,
    pub operacoes_tef: Option<String>,
    pub limite_desconto_acrescimo: Option<String>,
    pub sangria_lancamento_saida: Option<String>,
    pub desmembramento: Option<String>,
    pub libera_troco_maximo: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type UsuarioPermissao = Model;
