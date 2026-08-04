use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, sqlx::FromRow)]
#[sea_orm(table_name = "configuracoes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub pdv_numero: i32,
    pub empresa: i32,
    pub setor: i32,
    pub razao_social: Option<String>,
    pub nome_fantasia: Option<String>,
    pub cnpj: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub inscricao_municipal: Option<String>,
    pub cnae: Option<String>,
    pub codigo_regime_tributacao: i32,
    pub logradouro: Option<String>,
    pub complemento: Option<String>,
    pub numero: Option<String>,
    pub bairro: Option<String>,
    pub municipio: Option<String>,
    pub cod_municipio: i32,
    pub uf: Option<String>,
    pub cep: Option<String>,
    pub fone: Option<String>,
    pub mensagem_venda: Option<String>,
    pub exibir_valor_fechamento_caixa: Option<String>,
    pub exibir_valor_sangria: Option<String>,
    pub solicita_senha_venda: Option<String>,
    pub identifica_vendedor: Option<String>,
    pub diferenca_abastecimento: Decimal,
    pub quantidade_maxima_gerada: Decimal,
    pub quantidade_maxima_abastecimento: Decimal,
    pub tipo_estabelecimento: Option<String>,
    pub tipo_busca_abastecimento: i32,
    pub tipo_identificacao_cliente: i32,
    pub tipo_identificacao_fidelidade: i32,
    pub tipo_identificacao_usuario: i32,
    pub desconto_fechamento: Option<String>,
    pub imprime_gerencial_fidelidade: Option<String>,
    pub imprime_gerencial_promocao: Option<String>,
    pub imprime_espelho_completo: Option<String>,
    pub imprime_espelho_vencimento: Option<String>,
    pub imprime_recibo_espelho: Option<String>,
    pub imprime_rel_fechamento_caixa: Option<String>,
    pub imprime_rel_fechamento_turno: Option<String>,
    pub imprime_descricao_grade: Option<String>,
    pub imprime_espelho_sangria: Option<String>,
    pub imprime_espelho_suprimento: Option<String>,
    pub codigo_balanca: Option<String>,
    pub abre_venda_consulta_produto: Option<String>,
    pub vias_espelho: i32,
    pub pedido_agrupado: Option<String>,
    pub pre_venda_pagamento: Option<String>,
    pub alterar_pre_venda: Option<String>,
    pub atualizacao: Option<DateTime>,
    pub versao_retaguarda: Option<String>,
    pub senha_usuario_ativo: Option<String>,
    pub efetuar_sangria_usuario: Option<String>,
    pub valor_maximo_nfce: Decimal,
    pub exibir_limite_cliente: Option<String>,
    pub emissao_direta_nf_pj: Option<String>,
    pub lista_todos_abastecimentos_pdv: Option<String>,
    pub id_token: Option<String>,
    pub token_csc: Option<String>,
    pub controle_estoque_combustivel: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Configuracao = Model;
