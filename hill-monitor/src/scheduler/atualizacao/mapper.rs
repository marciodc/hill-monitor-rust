use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use rust_decimal::Decimal;
use uuid::Uuid;

use super::dto::*;

const TRUE_FLAG: &str = "T";
const FALSE_FLAG: &str = "F";
const ACTIVE_STATUS: &str = "A";

pub fn parse_datetime(s: &str) -> Option<chrono::NaiveDateTime> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Some(dt.naive_utc());
    }

    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.3fZ")
        .ok()
        .or_else(|| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.6fZ").ok())
        .or_else(|| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok())
}

fn tf(value: bool) -> String {
    if value {
        TRUE_FLAG.to_string()
    } else {
        FALSE_FLAG.to_string()
    }
}

fn one_char(value: Option<String>, default: &str) -> Option<String> {
    value
        .and_then(|s| s.chars().next().map(|c| c.to_string()))
        .or_else(|| Some(default.to_string()))
}

fn truncate_string(value: Option<String>, max_len: usize) -> Option<String> {
    value.map(|s| s.chars().take(max_len).collect())
}

fn extract_bico_retorno(codigo: Option<String>) -> Option<String> {
    let codigo = codigo?;
    let trimmed = codigo.trim();

    if let Some(rest) = trimmed.strip_prefix("B-") {
        let retorno = rest.split('.').next().unwrap_or(rest);
        return Some(retorno.chars().take(3).collect());
    }

    truncate_string(Some(codigo), 3)
}

fn normalize_bico_status(value: Option<String>) -> Option<String> {
    let normalized = value
        .as_deref()
        .map(|s| s.trim().to_ascii_uppercase())
        .and_then(|s| match s.as_str() {
            "A" | "ATIVO" => Some("A".to_string()),
            "I" | "INATIVO" | "INATIVVO" => Some("I".to_string()),
            _ => s.chars().next().map(|c| c.to_string()),
        });

    normalized.or_else(|| Some("A".to_string()))
}

fn tf_opt(value: Option<bool>) -> Option<String> {
    value.map(tf)
}

fn parse_decimal(value: Option<&str>) -> Decimal {
    value
        .and_then(|s| Decimal::from_str(s).ok())
        .unwrap_or(Decimal::ZERO)
}

fn parse_decimal_f64(value: Option<f64>) -> Decimal {
    value.and_then(Decimal::from_f64_retain).unwrap_or(Decimal::ZERO)
}

fn parse_i32_f64(value: Option<f64>) -> i32 {
    value.map(|v| v.trunc() as i32).unwrap_or(0)
}

fn parse_decimal_opt_string(value: &Option<String>) -> Decimal {
    parse_decimal(value.as_deref())
}

fn positive_balance(limit: Decimal, debit: Decimal) -> Decimal {
    if limit > debit {
        limit - debit
    } else {
        Decimal::ZERO
    }
}

fn selected_pdvs<'a>(pdvs: &'a [NewPdv], pdv_uuid: Option<Uuid>) -> Vec<&'a NewPdv> {
    match pdv_uuid {
        Some(id) => pdvs.iter().filter(|pdv| pdv.pdv_uuid == id).collect(),
        None => pdvs.iter().collect(),
    }
}

pub fn map_new_payload_to_sincronizacao(payload: NewSyncPayload, pdv_uuid: Option<Uuid>) -> Sincronizacao {
    let mut sinc = Sincronizacao {
        bicos: None,
        configuracoes: None,
        usuarios: None,
        produtos: None,
        moedas: None,
        parceiros: None,
        administradoras: Some(vec![]),
        parceiro_dependentes: None,
        parceiro_frotas: None,
        parceiro_formas_pagamento: None,
        parceiro_tabelas_formas_pagamento: None,
        parceiro_tabelas: None,
        setores: None,
        produtos_setores: None,
        tanques: None,
        tabela_precos: None,
        tabelapreco_itens: None,
        usuario_permissoes: None,
        vendedores: None,
    };

    let gerado_em = payload.gerado_em.as_deref().and_then(parse_datetime);
    let schema_version = payload.schema_version.clone();

    let cadastros = match payload.cadastros {
        Some(c) => c,
        None => return sinc,
    };

    let produtos_por_id: HashMap<i32, &NewProduto> = cadastros
        .produtos
        .as_ref()
        .map(|produtos| produtos.iter().map(|produto| (produto.id, produto)).collect())
        .unwrap_or_default();

    let tanques_por_id: HashMap<i32, &NewTanque> = cadastros
        .tanques
        .as_ref()
        .map(|tanques| tanques.iter().map(|tanque| (tanque.id, tanque)).collect())
        .unwrap_or_default();

    let grupos_ativos: HashSet<i32> = cadastros
        .grupos_permissao
        .as_ref()
        .map(|grupos| grupos.iter().filter(|g| g.ativo).map(|g| g.id).collect())
        .unwrap_or_default();

    let permissoes_por_grupo: HashMap<i32, HashSet<&str>> = cadastros
        .grupos_permissao_itens
        .as_ref()
        .map(|itens| {
            let mut grouped: HashMap<i32, HashSet<&str>> = HashMap::new();
            for item in itens {
                if item.permitido {
                    grouped
                        .entry(item.grupo_id)
                        .or_default()
                        .insert(item.recurso.as_str());
                }
            }
            grouped
        })
        .unwrap_or_default();

    if let (Some(pdvs), Some(empresa)) = (&cadastros.pdvs, &payload.empresa) {
        let mut mapped_configs = Vec::new();

        for pdv in selected_pdvs(pdvs, pdv_uuid) {
            let mut conf = hill_common::entity::Configuracao {
                id: pdv.pdv_uuid,
                pdv_numero: pdv.id,
                empresa: empresa.id,
                setor: 1,
                razao_social: empresa.razao_social.clone(),
                nome_fantasia: empresa.nome_fantasia.clone(),
                cnpj: empresa.cnpj.clone(),
                inscricao_estadual: empresa.inscricao_estadual.clone(),
                inscricao_municipal: empresa.inscricao_municipal.clone(),
                cnae: None,
                codigo_regime_tributacao: empresa.crt.unwrap_or(1),
                logradouro: empresa.endereco.clone(),
                complemento: None,
                numero: None,
                bairro: empresa.bairro.clone(),
                municipio: empresa.cidade.clone(),
                cod_municipio: empresa.codigo_ibge_municipio.unwrap_or(0),
                uf: empresa.estado.clone(),
                cep: empresa.cep.clone(),
                fone: empresa.telefone.clone(),
                mensagem_venda: None,
                exibir_valor_fechamento_caixa: None,
                exibir_valor_sangria: None,
                solicita_senha_venda: None,
                identifica_vendedor: None,
                diferenca_abastecimento: Decimal::ZERO,
                quantidade_maxima_gerada: Decimal::ZERO,
                quantidade_maxima_abastecimento: 0,
                tipo_estabelecimento: None,
                tipo_busca_abastecimento: 0,
                tipo_identificacao_cliente: 0,
                tipo_identificacao_fidelidade: 0,
                tipo_identificacao_usuario: 0,
                desconto_fechamento: None,
                imprime_gerencial_fidelidade: None,
                imprime_gerencial_promocao: None,
                imprime_espelho_completo: None,
                imprime_espelho_vencimento: None,
                imprime_recibo_espelho: None,
                imprime_rel_fechamento_caixa: None,
                imprime_rel_fechamento_turno: None,
                imprime_descricao_grade: Some(FALSE_FLAG.to_string()),
                imprime_espelho_sangria: None,
                imprime_espelho_suprimento: None,
                codigo_balanca: None,
                abre_venda_consulta_produto: None,
                vias_espelho: 1,
                pedido_agrupado: None,
                pre_venda_pagamento: None,
                alterar_pre_venda: None,
                atualizacao: gerado_em,
                versao_retaguarda: schema_version.clone(),
                senha_usuario_ativo: None,
                efetuar_sangria_usuario: None,
                vlr_max_nfce: Decimal::ZERO,
                exibir_limite_cliente: None,
                emissao_direta_nf_pj: None,
                lista_todos_abastecimentos_pdv: None,
                id_token: None,
                token_csc: None,
                controle_estoque_combustivel: None,
            };

            if let Some(pc) = &pdv.configuracao {
                conf.solicita_senha_venda = tf_opt(pc.solicita_senha_venda);
                conf.identifica_vendedor = tf_opt(pc.identifica_vendedor);
                conf.exibir_valor_fechamento_caixa = tf_opt(pc.exibir_valor_fechamento_caixa);
                conf.exibir_valor_sangria = tf_opt(pc.exibir_valor_sangria);
                conf.abre_venda_consulta_produto = tf_opt(pc.abre_venda_consulta_produto);
                conf.desconto_fechamento = tf_opt(pc.desconto_fechamento);
                conf.vlr_max_nfce = parse_decimal_f64(pc.vlr_max_nfce);
                conf.tipo_identificacao_fidelidade = pc.tipo_identificacao_fidelidade.unwrap_or(0);
                conf.tipo_busca_abastecimento = pc.tipo_busca_abastecimento.unwrap_or(0);
                conf.tipo_identificacao_usuario = pc.tipo_identificacao_usuario.unwrap_or(0);
                conf.tipo_identificacao_cliente = pc.tipo_identificacao_cliente.unwrap_or(0);
                conf.senha_usuario_ativo = tf_opt(pc.senha_usuario_ativo);
                conf.diferenca_abastecimento = parse_decimal_f64(pc.diferenca_abastecimento);
                conf.quantidade_maxima_gerada = parse_decimal_f64(pc.quantidade_maxima_gerada);
                conf.quantidade_maxima_abastecimento = parse_i32_f64(pc.quantidade_maxima_abastecimento);
                conf.controle_estoque_combustivel = tf_opt(pc.controle_estoque_combustivel);
                conf.lista_todos_abastecimentos_pdv = tf_opt(pc.lista_todos_abastecimentos_pdv);
                conf.imprime_recibo_espelho = tf_opt(pc.imprime_recibo_espelho);
                conf.imprime_espelho_completo = tf_opt(pc.imprime_espelho_completo);
                conf.imprime_gerencial_fidelidade = tf_opt(pc.imprime_gerencial_fidelidade);
                conf.imprime_gerencial_promocao = tf_opt(pc.imprime_gerencial_promocao);
                conf.imprime_espelho_sangria = tf_opt(pc.imprime_espelho_sangria);
                conf.imprime_espelho_suprimento = tf_opt(pc.imprime_espelho_suprimento);
                conf.imprime_rel_fechamento_caixa = tf_opt(pc.imprime_rel_fechamento_caixa);
                conf.imprime_rel_fechamento_turno = tf_opt(pc.imprime_rel_fechamento_turno);
                conf.imprime_espelho_vencimento = tf_opt(pc.imprime_espelho_vencimento);
                conf.vias_espelho = pc.vias_espelho.unwrap_or(1);
                conf.mensagem_venda = pc.mensagem_venda.clone();
                conf.efetuar_sangria_usuario = tf_opt(pc.efetuar_sangria_usuario);
                conf.exibir_limite_cliente = tf_opt(pc.exibir_limite_cliente);
                conf.emissao_direta_nf_pj = tf_opt(pc.emissao_direta_nf_pj);
            }

            mapped_configs.push(conf);
        }

        if !mapped_configs.is_empty() {
            sinc.configuracoes = Some(mapped_configs);
        }
    }

    if let Some(bicos_list) = &cadastros.bicos {
        let mut mapped_bicos = Vec::with_capacity(bicos_list.len());
        for b in bicos_list {
            let tank = tanques_por_id.get(&b.tanque_id).copied();
            let prod = tank.and_then(|t| produtos_por_id.get(&t.produto_id).copied());

            mapped_bicos.push(hill_common::entity::Bico {
                id: b.id,
                status: normalize_bico_status(b.status.clone()),
                retorno: extract_bico_retorno(b.codigo.clone()),
                numero: b.numero,
                bomba: b.bomba_id,
                tanque_id: b.tanque_id,
                bloqueio_quantidade: Decimal::ZERO,
                setor_id: 1,
                produto_id: tank.map(|t| t.produto_id).unwrap_or(0),
                gtin: prod.and_then(|p| p.codigo_barras.clone()),
                valor_unitario: parse_decimal_opt_string(&b.preco_unitario),
                valor_unitario_debito: parse_decimal_opt_string(&b.preco_unitario),
                valor_unitario_credito: parse_decimal_opt_string(&b.preco_unitario),
                combustivel: prod.map(|p| p.nome.clone()),
                altera_preco: one_char(b.sincroniza_preco_alterado.clone(), FALSE_FLAG),
                tabelapreco_id: 0,
                tipo_combustivel: prod
                    .and_then(|p| p.tipo_combustivel.clone())
                    .and_then(|s| s.chars().next().map(|c| c.to_string())),
                abastecimento_manual: Some("A".to_string()),
                bloqueado: one_char(b.bloqueado.clone(), FALSE_FLAG),
                sincroniza_preco_data_hora: b
                    .sincroniza_preco_data_hora
                    .as_deref()
                    .and_then(parse_datetime),
                sincroniza_preco_alterado: one_char(
                    b.sincroniza_preco_alterado.clone(),
                    FALSE_FLAG,
                ),
            });
        }
        sinc.bicos = Some(mapped_bicos);
    }

    if let Some(users_list) = &cadastros.usuarios_pdv {
        let mut mapped_users = Vec::with_capacity(users_list.len());
        for u in users_list {
            mapped_users.push(hill_common::entity::Usuario::try_from(u).unwrap());
        }
        sinc.usuarios = Some(mapped_users);
    }

    if let Some(prod_list) = &cadastros.produtos {
        let mut mapped_prod = Vec::with_capacity(prod_list.len());
        for p in prod_list {
            mapped_prod.push(hill_common::entity::Produto::try_from(p).unwrap());
        }
        sinc.produtos = Some(mapped_prod);
    }

    if let Some(fp_list) = &cadastros.formas_pagamento {
        let mut mapped_fp = Vec::with_capacity(fp_list.len());
        for fp in fp_list {
            mapped_fp.push(hill_common::entity::FormaPagamento::try_from(fp).unwrap());
        }
        sinc.moedas = Some(mapped_fp);
    }

    if let Some(cli_list) = &cadastros.clientes {
        let mut mapped_cli = Vec::with_capacity(cli_list.len());
        let mut mapped_parc_tab = Vec::new();

        for c in cli_list {
            let limite = parse_decimal_opt_string(&c.limite_credito);
            let saldo = parse_decimal_opt_string(&c.saldo_devedor);
            let status = if c.bloqueado {
                "B".to_string()
            } else if c.status_financeiro.as_deref() == Some("inativo") {
                "I".to_string()
            } else {
                ACTIVE_STATUS.to_string()
            };

            mapped_cli.push(hill_common::entity::Parceiro {
                id: c.id,
                status: Some(status),
                cpf_cnpj: c.cpf_cnpj.clone(),
                inscricao_estadual: c.ie.clone(),
                ie_situacao: c.ind_ie_dest.map(|v| v.to_string()),
                inscricao_municipal: None,
                nome_fantasia: Some(c.nome.clone()),
                razao_social: Some(c.nome.clone()),
                logradouro: c.logradouro.clone().or_else(|| c.endereco.clone()),
                complemento: c.complemento.clone(),
                numero: c.numero_endereco.clone(),
                bairro: c.bairro.clone(),
                municipio: c.cidade.clone(),
                cod_municipio: c.codigo_ibge_municipio.unwrap_or(0),
                uf: c.estado.clone(),
                cep: c.cep.clone(),
                requer_placa: Some(FALSE_FLAG.to_string()),
                requer_km: Some(FALSE_FLAG.to_string()),
                requer_condutor: Some(FALSE_FLAG.to_string()),
                desconto_venda: Decimal::ZERO,
                limite_disponivel: positive_balance(limite, saldo),
                email: c.email.clone(),
                rfid: c.cartao_rfid.clone(),
            });

            if let Some(tab_id) = c.tabela_preco_padrao_id {
                mapped_parc_tab.push(hill_common::entity::ParceiroTabela {
                    id: c.id,
                    status: ACTIVE_STATUS.to_string(),
                    parceiro_id: c.id,
                    tabela_id: tab_id,
                });
            }
        }

        sinc.parceiros = Some(mapped_cli);
        if !mapped_parc_tab.is_empty() {
            sinc.parceiro_tabelas = Some(mapped_parc_tab);
        }
    }

    if let Some(set_list) = &cadastros.setores {
        let mut mapped_set = Vec::with_capacity(set_list.len());
        for s in set_list {
            mapped_set.push(hill_common::entity::Setor::try_from(s).unwrap());
        }
        sinc.setores = Some(mapped_set);
    }

    if let Some(tanques_list) = &cadastros.tanques {
        let mut mapped_tanques = Vec::with_capacity(tanques_list.len());
        for t in tanques_list {
            let prod = produtos_por_id.get(&t.produto_id).copied();
            mapped_tanques.push(hill_common::entity::Tanque {
                id: t.id,
                numero: t.id,
                gtin: prod.and_then(|p| p.codigo_barras.clone()),
                descricao: t.nome.clone().or_else(|| t.codigo.clone()),
                capacidade: parse_decimal_opt_string(&t.capacidade_litros),
                estoque: parse_decimal_opt_string(&t.nivel_atual_litros),
            });
        }
        sinc.tanques = Some(mapped_tanques);
    }

    if let Some(tp_list) = &cadastros.tabelas_preco {
        let mut mapped_tp = Vec::with_capacity(tp_list.len());
        for tp in tp_list {
            mapped_tp.push(hill_common::entity::TabelaPreco::try_from(tp).unwrap());
        }
        sinc.tabela_precos = Some(mapped_tp);
    }

    if let Some(f_list) = &cadastros.frentistas {
        let mut mapped_vend = Vec::with_capacity(f_list.len());
        for f in f_list {
            mapped_vend.push(hill_common::entity::Vendedor::try_from(f).unwrap());
        }
        sinc.vendedores = Some(mapped_vend);
    }

    if let Some(ps_list) = &cadastros.produto_setores {
        let mut mapped_ps = Vec::with_capacity(ps_list.len());
        for ps in ps_list {
            mapped_ps.push(hill_common::entity::ProdutoSetor::try_from(ps).unwrap());
        }
        sinc.produtos_setores = Some(mapped_ps);
    }

    if let Some(cd_list) = &cadastros.clientes_dependentes {
        let mut mapped_cd = Vec::with_capacity(cd_list.len());
        for cd in cd_list {
            if let Ok(item) = hill_common::entity::ParceiroDependente::try_from(cd) {
                mapped_cd.push(item);
            }
        }
        sinc.parceiro_dependentes = Some(mapped_cd);
    }

    if let Some(cf_list) = &cadastros.clientes_frotas {
        let mut mapped_cf = Vec::with_capacity(cf_list.len());
        for cf in cf_list {
            if let Ok(item) = hill_common::entity::ParceiroFrota::try_from(cf) {
                mapped_cf.push(item);
            }
        }
        sinc.parceiro_frotas = Some(mapped_cf);
    }

    if let Some(cfp_list) = &cadastros.clientes_formas_pagamento {
        let mut mapped_cfp = Vec::with_capacity(cfp_list.len());
        for cfp in cfp_list {
            if let Ok(item) = hill_common::entity::ParceiroFormaPagamento::try_from(cfp) {
                mapped_cfp.push(item);
            }
        }
        sinc.parceiro_formas_pagamento = Some(mapped_cfp);
    }

    if let Some(cfpt_list) = &cadastros.clientes_formas_pagamento_tabelas {
        let mut mapped_cfpt = Vec::with_capacity(cfpt_list.len());
        for cfpt in cfpt_list {
            if let Ok(item) = hill_common::entity::ParceiroTabelaFormaPagamento::try_from(cfpt) {
                mapped_cfpt.push(item);
            }
        }
        sinc.parceiro_tabelas_formas_pagamento = Some(mapped_cfpt);
    }

    if let Some(tpi_list) = &cadastros.tabelas_preco_itens {
        let mut mapped_tpi = Vec::with_capacity(tpi_list.len());
        for tpi in tpi_list {
            mapped_tpi.push(hill_common::entity::TabelaPrecoItem::try_from(tpi).unwrap());
        }
        sinc.tabelapreco_itens = Some(mapped_tpi);
    }

    if let Some(users_list) = &cadastros.usuarios_pdv {
        let mut mapped_perm = Vec::with_capacity(users_list.len());
        for u in users_list {
            let is_admin = u.papel.as_deref() == Some("admin");
            let mut p = hill_common::entity::UsuarioPermissao {
                id: u.user_id,
                usuario_id: u.user_id,
                cancela_venda_aberta: Some(FALSE_FLAG.to_string()),
                cancela_venda_fechada: Some(FALSE_FLAG.to_string()),
                cancela_item: Some(FALSE_FLAG.to_string()),
                desconto_item: Some(FALSE_FLAG.to_string()),
                desconto_fechamento: Some(FALSE_FLAG.to_string()),
                desconto_fechamento_pv: Some(FALSE_FLAG.to_string()),
                acrescimo_fechamento: Some(FALSE_FLAG.to_string()),
                acrescimo_item: Some(FALSE_FLAG.to_string()),
                acrescimo_fechamento_pv: Some(FALSE_FLAG.to_string()),
                cliente_limite: Some(FALSE_FLAG.to_string()),
                cliente_bloqueado: Some(FALSE_FLAG.to_string()),
                cliente_forma_pagamento: Some(FALSE_FLAG.to_string()),
                sangria: Some(FALSE_FLAG.to_string()),
                suprimento: Some(FALSE_FLAG.to_string()),
                abertura_turno: Some(FALSE_FLAG.to_string()),
                fechamento_turno: Some(FALSE_FLAG.to_string()),
                reabertura_turno: Some(FALSE_FLAG.to_string()),
                afericao: Some(FALSE_FLAG.to_string()),
                lista_todos_abastecimentos: Some(FALSE_FLAG.to_string()),
                operacoes_tef: Some(FALSE_FLAG.to_string()),
                limite_desconto_acrescimo: Some(FALSE_FLAG.to_string()),
                sangria_lancamento_saida: Some(FALSE_FLAG.to_string()),
                desmembramento: Some(FALSE_FLAG.to_string()),
                libera_troco_maximo: Some(FALSE_FLAG.to_string()),
            };

            if is_admin {
                let all_true = Some(TRUE_FLAG.to_string());
                p.cancela_venda_aberta = all_true.clone();
                p.cancela_venda_fechada = all_true.clone();
                p.cancela_item = all_true.clone();
                p.desconto_item = all_true.clone();
                p.desconto_fechamento = all_true.clone();
                p.desconto_fechamento_pv = all_true.clone();
                p.acrescimo_fechamento = all_true.clone();
                p.acrescimo_item = all_true.clone();
                p.acrescimo_fechamento_pv = all_true.clone();
                p.cliente_limite = all_true.clone();
                p.cliente_bloqueado = all_true.clone();
                p.cliente_forma_pagamento = all_true.clone();
                p.sangria = all_true.clone();
                p.suprimento = all_true.clone();
                p.abertura_turno = all_true.clone();
                p.fechamento_turno = all_true.clone();
                p.reabertura_turno = all_true.clone();
                p.afericao = all_true.clone();
                p.lista_todos_abastecimentos = all_true.clone();
                p.operacoes_tef = all_true.clone();
                p.limite_desconto_acrescimo = all_true.clone();
                p.sangria_lancamento_saida = all_true.clone();
                p.desmembramento = all_true.clone();
                p.libera_troco_maximo = all_true;
            } else if let Some(gid) = u.grupo_permissao_id.filter(|gid| grupos_ativos.contains(gid)) {
                if let Some(recursos) = permissoes_por_grupo.get(&gid) {
                    let has_rec = |rec: &str| recursos.contains(rec);

                    p.cancela_venda_aberta = Some(tf(has_rec("pdv_cancelar_cupom")));
                    p.cancela_venda_fechada = Some(tf(has_rec("vendas_cancelar")));
                    p.cancela_item = Some(tf(has_rec("pdv_cancelar_cupom")));
                    p.desconto_item = Some(tf(has_rec("pdv_liberar_desconto")));
                    p.desconto_fechamento = Some(tf(has_rec("pdv_liberar_desconto")));
                    p.desconto_fechamento_pv = Some(tf(has_rec("pdv_liberar_desconto")));
                    p.acrescimo_item = Some(tf(has_rec("pdv_liberar_acrescimo")));
                    p.acrescimo_fechamento = Some(tf(has_rec("pdv_liberar_acrescimo")));
                    p.acrescimo_fechamento_pv = Some(tf(has_rec("pdv_liberar_acrescimo")));
                    p.cliente_limite = Some(tf(has_rec("pdv_liberar_limite_cliente")));
                    p.cliente_bloqueado = Some(tf(has_rec("pdv_liberar_cliente_bloqueado")));
                    p.cliente_forma_pagamento = Some(tf(has_rec("pdv_alterar_forma_pagamento")));
                    p.sangria = Some(tf(has_rec("pdv_sangria_suprimento")));
                    p.suprimento = Some(tf(has_rec("pdv_sangria_suprimento")));
                    p.abertura_turno = Some(tf(has_rec("pdv_abrir_caixa")));
                    p.fechamento_turno = Some(tf(has_rec("pdv_fechar_caixa")));
                    p.reabertura_turno = Some(tf(has_rec("pdv_reabrir_caixa")));
                    p.afericao = Some(tf(has_rec("pdv_afericao")));
                    p.lista_todos_abastecimentos = Some(tf(has_rec("pdv_carregar_pre_venda")));
                    p.operacoes_tef = Some(tf(has_rec("pdv_operacoes_tef")));
                    p.limite_desconto_acrescimo =
                        Some(tf(has_rec("pdv_limite_desconto_acrescimo")));
                    p.sangria_lancamento_saida =
                        Some(tf(has_rec("pdv_sangria_lancamento_saida")));
                    p.desmembramento = Some(tf(has_rec("pdv_desmembramento")));
                    p.libera_troco_maximo = Some(tf(has_rec("pdv_libera_troco_maximo")));
                }
            }

            mapped_perm.push(p);
        }
        sinc.usuario_permissoes = Some(mapped_perm);
    }

    sinc
}
