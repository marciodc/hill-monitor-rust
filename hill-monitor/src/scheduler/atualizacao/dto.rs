use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

const TRUE_FLAG: &str = "T";
const FALSE_FLAG: &str = "F";
const ACTIVE_STATUS: &str = "A";
const INACTIVE_STATUS: &str = "I";

fn tf(value: bool) -> String {
    if value {
        TRUE_FLAG.to_string()
    } else {
        FALSE_FLAG.to_string()
    }
}

fn tf_opt(value: Option<bool>) -> Option<String> {
    value.map(tf)
}

fn status_flag(active: bool) -> String {
    if active {
        ACTIVE_STATUS.to_string()
    } else {
        INACTIVE_STATUS.to_string()
    }
}

fn parse_decimal(value: Option<&str>) -> Decimal {
    value
        .and_then(|s| s.parse::<Decimal>().ok())
        .unwrap_or(Decimal::ZERO)
}

fn parse_decimal_f64(value: Option<f64>) -> Decimal {
    value.and_then(Decimal::from_f64_retain).unwrap_or(Decimal::ZERO)
}

fn parse_i32(value: Option<&str>, default: i32) -> i32 {
    value.and_then(|s| s.parse::<i32>().ok()).unwrap_or(default)
}

fn positive_balance(limit: Decimal, debit: Decimal) -> Decimal {
    if limit > debit {
        limit - debit
    } else {
        Decimal::ZERO
    }
}

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
    pub codigo: Option<String>,
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

impl TryFrom<&NewUsuarioPdv> for hill_common::entity::Usuario {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewUsuarioPdv) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.user_id,
            status: ACTIVE_STATUS.to_string(),
            nome: value.nome.clone(),
            login: value
                .login_pdv
                .clone()
                .unwrap_or_else(|| value.user_id.to_string()),
            senha: value.senha_pdv_hash.clone().unwrap_or_default(),
            rfid: None,
            rfid_debito: None,
            rfid_credito: None,
            digital: None,
            cartao_magnetico: None,
            perc_max_desc_acres_item: Decimal::ZERO,
            valor_max_desc_acres_item: Decimal::ZERO,
            perc_max_desc_acres_subtotal: Decimal::ZERO,
            valor_max_desc_acres_subtotal: Decimal::ZERO,
        })
    }
}

impl TryFrom<&NewProduto> for hill_common::entity::Produto {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewProduto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            tipo: Some(if value.is_combustivel {
                "C".to_string()
            } else {
                "P".to_string()
            }),
            categoria: None,
            unidade_tributacao: None,
            descricao: value.nome.clone(),
            descricao_resumida: None,
            gtin_tributacao: value.codigo_barras.clone(),
            gtin_comercial: value.codigo_barras.clone(),
            unidade_comercial: None,
            quantidade_tributacao: Decimal::ZERO,
            ncm: value.ncm.clone(),
            ncm_excecao: None,
            imposto_aliquota_importacao: Decimal::ZERO,
            imposto_aliquota_federal: Decimal::ZERO,
            imposto_aliquota_estadual: Decimal::ZERO,
            imposto_aliquota_municipal: Decimal::ZERO,
            imposto_chave: None,
            tipo_codigo: 0,
            codigo: Some(value.codigo.clone()),
            codigo_auxiliar: None,
            indicador_producao: None,
            fracionado: None,
            pesado_caixa: None,
            cst: value.cst_icms_padrao.clone(),
            cst_pis: value.cst_pis.clone(),
            cst_cofins: value.cst_cofins.clone(),
            observacao: None,
            codigo_anp: 0,
            descricao_anp: None,
            solicita_vendedor: None,
            grade_id: None,
            controla_numero_serie: None,
            controla_lote: None,
            setor_impressao_1: None,
            setor_impressao_2: None,
            setor_impressao_3: None,
            setor_impressao_4: None,
            exclusivo_kit: None,
            cest: value.cest.clone(),
            cfop: 0,
            aliquota: parse_decimal_f64(value.aliquota_icms_simples),
            aliquota_cofins: parse_decimal_f64(value.aliq_cofins_pct),
            aliquota_pis: parse_decimal_f64(value.aliq_pis_pct),
            tipo_combustivel: parse_i32(value.tipo_combustivel.as_deref(), 0),
            etiqueta_balanca: None,
            predbcefet: None,
            picmsefet: None,
            pfcpstret: None,
            pfcpst: None,
            pfcp: None,
            modbc: None,
            modbcst: None,
            pmvast: None,
            predbcst: None,
            picmsst: None,
            predbc: None,
            pglp: None,
            pgnn: None,
            pgni: None,
            vpart: None,
        })
    }
}

impl TryFrom<&NewFormaPagamento> for hill_common::entity::FormaPagamento {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewFormaPagamento) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            numero: value.id,
            tipo_pagamento: parse_i32(value.nfce_tpag.as_deref(), 99),
            descricao: value.descricao.clone(),
            valor_aviso_sangria: parse_decimal(value.limite_sangria_aviso.as_deref()),
            somente_cadastrados: tf_opt(value.requer_cliente),
            permite_troco: Some(TRUE_FLAG.to_string()),
            permite_desconto: Some(TRUE_FLAG.to_string()),
            permite_acrescimo: Some(TRUE_FLAG.to_string()),
            dados_cheque: Some(FALSE_FLAG.to_string()),
            dados_tef: tf_opt(value.chama_tef),
            maximo_parcelas: if value.permite_parcelamento.unwrap_or(false) {
                12
            } else {
                1
            },
            tef_rede: None,
            tef_operacao: 0,
            voucher: Some(FALSE_FLAG.to_string()),
            ignora_limite_troco: Some(TRUE_FLAG.to_string()),
            solicita_vencimento: Some(FALSE_FLAG.to_string()),
            valida_limite_credito: Some(TRUE_FLAG.to_string()),
            espelho: Some(FALSE_FLAG.to_string()),
            dias_vencimento: None,
            tipo_venda: Some(if value.liquidacao.as_deref() == Some("aprazo") {
                "P".to_string()
            } else {
                "V".to_string()
            }),
            tabela_id: 0,
            permite_cheque_troco: Some(FALSE_FLAG.to_string()),
            permite_deposito_troco: Some(FALSE_FLAG.to_string()),
            percentual_maximo_troco: Decimal::ZERO,
            percentual_desconto: Decimal::ZERO,
            percentual_maximo_desconto: Decimal::ZERO,
            venda_mobile: Some(FALSE_FLAG.to_string()),
            troco_em_deposito: Some(FALSE_FLAG.to_string()),
            vendas_com_juros_mobile: Some(FALSE_FLAG.to_string()),
        })
    }
}

impl TryFrom<&NewSetor> for hill_common::entity::Setor {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewSetor) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            descricao: value.nome.clone(),
        })
    }
}

impl TryFrom<&NewTabelaPreco> for hill_common::entity::TabelaPreco {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewTabelaPreco) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            status: status_flag(value.ativo),
            padrao: tf(value.id == 1),
            descricao: value.nome.clone(),
            exclusiva_cliente: Some(FALSE_FLAG.to_string()),
        })
    }
}

impl TryFrom<&NewFrentista> for hill_common::entity::Vendedor {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewFrentista) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            codigo: parse_i32(value.codigo_frentista.as_deref(), value.id),
            nome: value.nome.clone(),
        })
    }
}

impl TryFrom<&NewProdutoSetor> for hill_common::entity::ProdutoSetor {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewProdutoSetor) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            setor_id: value.setor_id,
            produto_id: value.produto_id,
        })
    }
}

impl TryFrom<&NewClienteDependente> for hill_common::entity::ParceiroDependente {
    type Error = &'static str;

    fn try_from(value: &NewClienteDependente) -> Result<Self, Self::Error> {
        let limite = parse_decimal(value.limite_credito.as_deref());
        let saldo = parse_decimal(value.saldo_devedor.as_deref());

        Ok(Self {
            id: value.id,
            status: status_flag(value.ativo),
            parceiro_id: value.cliente_id.ok_or("cliente_id ausente")?,
            nome: value.nome.clone(),
            rfid: value.cartao_rfid.clone(),
            limite_disponivel: positive_balance(limite, saldo),
        })
    }
}

impl TryFrom<&NewClienteFrota> for hill_common::entity::ParceiroFrota {
    type Error = &'static str;

    fn try_from(value: &NewClienteFrota) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            status: status_flag(value.ativo),
            parceiro_id: value.cliente_id.ok_or("cliente_id ausente")?,
            veiculo: value.veiculo_descricao.clone().unwrap_or_default(),
            placa: value.placa.clone(),
        })
    }
}

impl TryFrom<&NewClienteFormaPagamento> for hill_common::entity::ParceiroFormaPagamento {
    type Error = &'static str;

    fn try_from(value: &NewClienteFormaPagamento) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            parceiro_id: value.cliente_id.ok_or("cliente_id ausente")?,
            forma_pagamento_id: value
                .forma_pagamento_id
                .ok_or("forma_pagamento_id ausente")?,
        })
    }
}

impl TryFrom<&NewClienteFormaPagamentoTabela> for hill_common::entity::ParceiroTabelaFormaPagamento {
    type Error = &'static str;

    fn try_from(value: &NewClienteFormaPagamentoTabela) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            status: status_flag(value.ativo),
            parceiro_id: value.cliente_id.ok_or("cliente_id ausente")?,
            forma_pagamento_id: value
                .forma_pagamento_id
                .ok_or("forma_pagamento_id ausente")?,
            tabela_id: value.tabela_id.ok_or("tabela_id ausente")?,
        })
    }
}

impl TryFrom<&NewTabelaPrecoItem> for hill_common::entity::TabelaPrecoItem {
    type Error = std::convert::Infallible;

    fn try_from(value: &NewTabelaPrecoItem) -> Result<Self, Self::Error> {
        let preco = parse_decimal(value.preco_venda.as_deref());
        Ok(Self {
            id: value.id,
            tabela_preco_id: value.tabela_id,
            produto_id: value.produto_id,
            valor_comercial: preco,
            valor_tributacao: preco,
        })
    }
}
