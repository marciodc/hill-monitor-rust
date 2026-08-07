use serde::{Deserialize, Deserializer};
use uuid::Uuid;

fn de_opt_f64_from_string_or_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum F64OrString {
        F64(f64),
        String(String),
    }

    let value = Option::<F64OrString>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(F64OrString::F64(v)) => Ok(Some(v)),
        Some(F64OrString::String(s)) => Ok(s.parse::<f64>().ok()),
    }
}

#[derive(Deserialize)]
pub struct NewSyncPayload {
    pub ok: bool,
    pub schema_version: Option<String>,
    pub gerado_em: Option<String>,
    pub empresa: Option<NewEmpresa>,
    pub cadastros: Option<NewCadastros>,
}

pub struct Sincronizacao {
    pub bicos: Option<Vec<hill_common::entity::Bico>>,
    pub configuracoes: Option<Vec<hill_common::entity::Configuracao>>,
    pub usuarios: Option<Vec<hill_common::entity::Usuario>>,
    pub produtos: Option<Vec<hill_common::entity::Produto>>,
    pub moedas: Option<Vec<hill_common::entity::FormaPagamento>>,
    pub parceiros: Option<Vec<hill_common::entity::Parceiro>>,
    pub administradoras: Option<Vec<hill_common::entity::Administradora>>,
    pub parceiro_dependentes: Option<Vec<hill_common::entity::ParceiroDependente>>,
    pub parceiro_frotas: Option<Vec<hill_common::entity::ParceiroFrota>>,
    pub parceiro_formas_pagamento: Option<Vec<hill_common::entity::ParceiroFormaPagamento>>,
    pub parceiro_tabelas_formas_pagamento:
        Option<Vec<hill_common::entity::ParceiroTabelaFormaPagamento>>,
    pub parceiro_tabelas: Option<Vec<hill_common::entity::ParceiroTabela>>,
    pub setores: Option<Vec<hill_common::entity::Setor>>,
    pub produtos_setores: Option<Vec<hill_common::entity::ProdutoSetor>>,
    pub tanques: Option<Vec<hill_common::entity::Tanque>>,
    pub tabela_precos: Option<Vec<hill_common::entity::TabelaPreco>>,
    pub tabelapreco_itens: Option<Vec<hill_common::entity::TabelaPrecoItem>>,
    pub usuario_permissoes: Option<Vec<hill_common::entity::UsuarioPermissao>>,
    pub vendedores: Option<Vec<hill_common::entity::Vendedor>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct NewEmpresa {
    pub id: i32,
    pub nome: Option<String>,
    pub razao_social: Option<String>,
    pub nome_fantasia: Option<String>,
    pub cnpj: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub inscricao_municipal: Option<String>,
    pub cnae: Option<String>,
    pub regime_tributario: Option<String>,
    pub crt: Option<i32>,
    pub endereco: Option<String>,
    pub bairro: Option<String>,
    pub cidade: Option<String>,
    pub estado: Option<String>,
    pub cep: Option<String>,
    pub telefone: Option<String>,
    pub codigo_ibge_municipio: Option<i32>,
}

#[derive(Deserialize)]
pub struct NewCadastros {
    pub pdvs: Option<Vec<NewPdv>>,
    pub bicos: Option<Vec<NewBico>>,
    pub produtos: Option<Vec<NewProduto>>,
    pub produto_setores: Option<Vec<NewProdutoSetor>>,
    pub setores: Option<Vec<NewSetor>>,
    pub tanques: Option<Vec<NewTanque>>,
    pub frentistas: Option<Vec<NewFrentista>>,
    pub clientes: Option<Vec<NewCliente>>,
    pub formas_pagamento: Option<Vec<NewFormaPagamento>>,
    pub clientes_dependentes: Option<Vec<NewClienteDependente>>,
    pub clientes_frotas: Option<Vec<NewClienteFrota>>,
    pub clientes_formas_pagamento: Option<Vec<NewClienteFormaPagamento>>,
    pub clientes_formas_pagamento_tabelas: Option<Vec<NewClienteFormaPagamentoTabela>>,
    pub tabelas_preco: Option<Vec<NewTabelaPreco>>,
    pub tabelas_preco_itens: Option<Vec<NewTabelaPrecoItem>>,
    pub usuarios_pdv: Option<Vec<NewUsuarioPdv>>,
    pub grupos_permissao: Option<Vec<NewGrupoPermissao>>,
    pub grupos_permissao_itens: Option<Vec<NewGrupoPermissaoItem>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct NewPdv {
    pub id: i32,
    pub codigo: Option<String>,
    pub pdv_uuid: Uuid,
    pub configuracao: Option<NewPdvConfig>,
}

#[derive(Deserialize)]
pub struct NewPdvConfig {
    pub solicita_senha_venda: Option<bool>,
    pub identifica_vendedor: Option<bool>,
    pub exibir_valor_fechamento_caixa: Option<bool>,
    pub exibir_valor_sangria: Option<bool>,
    pub abre_venda_consulta_produto: Option<bool>,
    pub desconto_fechamento: Option<bool>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub vlr_max_nfce: Option<f64>,
    pub tipo_identificacao_fidelidade: Option<i32>,
    pub tipo_busca_abastecimento: Option<i32>,
    pub tipo_identificacao_usuario: Option<i32>,
    pub tipo_identificacao_cliente: Option<i32>,
    pub senha_usuario_ativo: Option<bool>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub diferenca_abastecimento: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub quantidade_maxima_gerada: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub quantidade_maxima_abastecimento: Option<f64>,
    pub controle_estoque_combustivel: Option<bool>,
    pub lista_todos_abastecimentos_pdv: Option<bool>,
    pub imprime_recibo_espelho: Option<bool>,
    pub imprime_espelho_completo: Option<bool>,
    pub imprime_gerencial_fidelidade: Option<bool>,
    pub imprime_gerencial_promocao: Option<bool>,
    pub imprime_espelho_sangria: Option<bool>,
    pub imprime_espelho_suprimento: Option<bool>,
    pub imprime_rel_fechamento_caixa: Option<bool>,
    pub imprime_rel_fechamento_turno: Option<bool>,
    pub imprime_espelho_vencimento: Option<bool>,
    pub vias_espelho: Option<i32>,
    pub mensagem_venda: Option<String>,
    pub efetuar_sangria_usuario: Option<bool>,
    pub exibir_limite_cliente: Option<bool>,
    pub emissao_direta_nf_pj: Option<bool>,
}

#[derive(Deserialize)]
pub struct NewBico {
    pub id: i32,
    pub bomba_id: i32,
    pub tanque_id: i32,
    pub numero: i32,
    #[allow(dead_code)]
    pub codigo: Option<String>,
    pub retorno: Option<String>,
    pub preco_unitario: Option<String>,
    pub status: Option<String>,
    pub bloqueado: Option<String>,
    pub sincroniza_preco_alterado: Option<String>,
    pub sincroniza_preco_data_hora: Option<String>,
}

#[derive(Deserialize)]
pub struct NewProduto {
    pub id: i32,
    pub ativo: bool,
    pub categoria_id: Option<i32>,
    pub codigo_barras: Option<String>,
    pub codigo: String,
    pub nome: String,
    pub is_combustivel: bool,
    pub tipo_combustivel: Option<String>,
    pub preco_venda: Option<String>,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub cst_icms_padrao: Option<String>,
    pub csosn_padrao: Option<String>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub aliquota_icms_simples: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub aliq_pis_pct: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_f64_from_string_or_number")]
    pub aliq_cofins_pct: Option<f64>,
    pub cst_pis: Option<String>,
    pub cst_cofins: Option<String>,
}

#[derive(Deserialize)]
pub struct NewProdutoSetor {
    pub id: i32,
    pub setor_id: i32,
    pub produto_id: i32,
}

#[derive(Deserialize)]
pub struct NewSetor {
    pub id: i32,
    pub nome: String,
}

#[derive(Deserialize)]
pub struct NewTanque {
    pub id: i32,
    pub produto_id: i32,
    pub codigo: Option<String>,
    pub nome: Option<String>,
    pub capacidade_litros: Option<String>,
    pub nivel_atual_litros: Option<String>,
}

#[derive(Deserialize)]
pub struct NewFrentista {
    pub id: i32,
    pub nome: String,
    pub codigo_frentista: Option<String>,
}

#[derive(Deserialize)]
pub struct NewCliente {
    pub id: i32,
    pub nome: String,
    pub cpf_cnpj: Option<String>,
    pub email: Option<String>,
    pub cidade: Option<String>,
    pub estado: Option<String>,
    pub cep: Option<String>,
    pub logradouro: Option<String>,
    pub endereco: Option<String>,
    pub complemento: Option<String>,
    pub numero_endereco: Option<String>,
    pub bairro: Option<String>,
    pub codigo_ibge_municipio: Option<i32>,
    pub ie: Option<String>,
    pub ind_ie_dest: Option<i32>,
    pub limite_credito: Option<String>,
    pub saldo_devedor: Option<String>,
    pub bloqueado: bool,
    pub tabela_preco_padrao_id: Option<i32>,
    pub cartao_rfid: Option<String>,
    pub status_financeiro: Option<String>,
}

#[derive(Deserialize)]
pub struct NewFormaPagamento {
    pub id: i32,
    pub descricao: String,
    pub liquidacao: Option<String>,
    pub instrumento: Option<String>,
    pub permite_parcelamento: Option<bool>,
    pub limite_sangria_aviso: Option<String>,
    pub requer_cliente: Option<bool>,
    pub chama_tef: Option<bool>,
    pub nfce_tpag: Option<String>,
}

#[derive(Deserialize)]
pub struct NewClienteDependente {
    pub id: i32,
    pub cliente_id: Option<i32>,
    pub nome: String,
    pub cartao_rfid: Option<String>,
    pub limite_credito: Option<String>,
    pub saldo_devedor: Option<String>,
    pub ativo: bool,
}

#[derive(Deserialize)]
pub struct NewClienteFrota {
    pub id: i32,
    pub cliente_id: Option<i32>,
    pub veiculo_descricao: Option<String>,
    pub placa: Option<String>,
    pub ativo: bool,
}

#[derive(Deserialize)]
pub struct NewClienteFormaPagamento {
    pub id: i32,
    pub cliente_id: Option<i32>,
    pub forma_pagamento_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct NewClienteFormaPagamentoTabela {
    pub id: i32,
    pub cliente_id: Option<i32>,
    pub forma_pagamento_id: Option<i32>,
    pub tabela_id: Option<i32>,
    pub ativo: bool,
}

#[derive(Deserialize)]
pub struct NewTabelaPreco {
    pub id: i32,
    pub nome: String,
    pub ativo: bool,
}

#[derive(Deserialize)]
pub struct NewTabelaPrecoItem {
    pub id: i32,
    pub tabela_id: i32,
    pub produto_id: i32,
    pub preco_venda: Option<String>,
}

#[derive(Deserialize)]
pub struct NewUsuarioPdv {
    pub user_id: i32,
    pub nome: String,
    pub login_pdv: Option<String>,
    pub senha_pdv_hash: Option<String>,
    pub papel: Option<String>,
    pub grupo_permissao_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct NewGrupoPermissaoItem {
    pub grupo_id: i32,
    pub recurso: String,
    pub permitido: bool,
}

#[derive(Deserialize)]
pub struct NewGrupoPermissao {
    pub id: i32,
    pub ativo: bool,
}
