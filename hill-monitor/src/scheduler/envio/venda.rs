use crate::backend_url::sync_send_url;
use hill_common::entity::{abastecimento, venda, venda_item, venda_pagamento};
use hill_common::net::HttpConn;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use tracing::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct SincronizacaoResponse {
    venda: Option<SincronizacaoResult>,
}

#[derive(serde::Deserialize)]
struct SincronizacaoResult {
    result: Option<String>,
}

pub async fn envia_vendas(
    db: &DatabaseConnection,
    http: &HttpConn,
    backend_url: &str,
    token: &str,
    empresa_id: i32,
    tipo_estabelecimento: &str,
) -> Result<(), sea_orm::DbErr> {
    let vendas = match venda::Entity::find()
        .filter(
            venda::Column::Estorno
                .ne("T")
                .and(venda::Column::Finalizada.eq("T"))
                .and(
                    venda::Column::Sincronizado
                        .ne("T")
                        .or(venda::Column::Sincronizado.is_null()),
                ),
        )
        .order_by_asc(venda::Column::Id)
        .limit(10)
        .all(db)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            error!(
                "EnvioDadosVenda - Erro ao buscar vendas não sincronizadas: {:?}",
                e
            );
            return Err(e);
        }
    };

    for v in vendas {
        let id = v.id;

        // Fetch items
        let items = match venda_item::Entity::find()
            .filter(venda_item::Column::VendaId.eq(id))
            .all(db)
            .await
        {
            Ok(list) => list,
            _ => Vec::new(),
        };

        // Fetch payments
        let payments = match venda_pagamento::Entity::find()
            .filter(venda_pagamento::Column::VendaId.eq(id))
            .all(db)
            .await
        {
            Ok(list) => list,
            _ => Vec::new(),
        };

        // Fetch linked abastecimentos
        let abast_ids: Vec<Uuid> = items
            .iter()
            .filter_map(|item| item.abastecimento_id)
            .collect();
        let linked_abast = if !abast_ids.is_empty() {
            abastecimento::Entity::find()
                .filter(abastecimento::Column::Id.is_in(abast_ids))
                .all(db)
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        // Convert items to JSON list
        let items_json: Vec<serde_json::Value> = items.iter().map(|item| {
            let status_val = if v.status == "C" { "C" } else { item.status.as_deref().unwrap_or("") };
            serde_json::json!({
                "status": status_val,
                "sequencia": item.sequencia.to_string(),
                "produto_id": item.produto_id.to_string(),
                "quantidade": format!("{:.3}", item.quantidade).replace('.', ","),
                "valor": format!("{:.2}", item.valor_comercial).replace('.', ","),
                "subtotal": format!("{:.2}", item.subtotal).replace('.', ","),
                "desconto": format!("{:.2}", item.desconto).replace('.', ","),
                "acrescimo": format!("{:.2}", item.acrescimo).replace('.', ","),
                "total": format!("{:.2}", item.total).replace('.', ","),
                "cst": item.cst.as_deref().unwrap_or(""),
                "cfop": item.cfop.to_string(),
                "icms_aliquota": format!("{:.2}", item.icms_aliquota).replace('.', ","),
                "icms_valor": format!("{:.2}", item.icms_valor).replace('.', ","),
                "cst_pis": item.cst_pis.as_deref().unwrap_or(""),
                "pis_aliquota": format!("{:.2}", item.pis_aliquota).replace('.', ","),
                "pis_valor": format!("{:.2}", item.pis_valor).replace('.', ","),
                "cst_cofins": item.cst_cofins.as_deref().unwrap_or(""),
                "cofins_aliquota": format!("{:.2}", item.cofins_aliquota).replace('.', ","),
                "cofins_valor": format!("{:.2}", item.cofins_valor).replace('.', ","),
                "abastecimento_id": item.abastecimento_id.map(|uid| uid.to_string()).unwrap_or_default(),
                "bico_id": item.bico_id.to_string(),
                "encerrante_inicial": format!("{:.3}", item.encerrante_inicial).replace('.', ","),
                "encerrante_final": format!("{:.3}", item.encerrante_final).replace('.', ","),
                "rfid_vendedor": item.rfid_vendedor.as_deref().unwrap_or(""),
                "rfid_cliente": item.rfid_cliente.as_deref().unwrap_or(""),
                "setor_id": item.setor_id.to_string(),
                "grade_item_id": item.grade_item_id.to_string(),
                "grade_codigo": item.grade_codigo.as_deref().unwrap_or(""),
                "lote_id": if item.lote_id > rust_decimal::Decimal::ZERO { item.lote_id.to_string() } else { String::new() },
                "tabela_preco_id": item.tabela_preco_id.to_string(),
                "produto_serie_id": item.produto_serie_id.to_string(),
            })
        }).collect();

        // Convert payments to JSON list
        let payments_json: Vec<serde_json::Value> = payments.iter().map(|p| {
            serde_json::json!({
                "forma_pagamento_id": p.forma_pagamento_id.to_string(),
                "subtotal": format!("{:.2}", p.subtotal).replace('.', ","),
                "desconto": "0,00",
                "acrescimo": "0,00",
                "total": format!("{:.2}", p.total).replace('.', ","),
                "recebido": format!("{:.2}", p.recebido).replace('.', ","),
                "troco": format!("{:.2}", p.troco).replace('.', ","),
                "data_vencimento": p.data_vencimento.map(|dt| dt.format("%d/%m/%Y").to_string()).unwrap_or_default(),
                "voucher_id": if p.voucher_id.unwrap_or(0) > 0 { p.voucher_id.unwrap().to_string() } else { String::new() },
                "tef_terminal": p.tef_terminal.as_deref().unwrap_or(""),
                "tef_nsu": p.tef_nsu.as_deref().unwrap_or(""),
                "tef_rede": p.tef_rede.as_deref().unwrap_or(""),
                "tef_bandeira": p.tef_bandeira.as_deref().unwrap_or(""),
                "tef_operacao": p.tef_operacao.as_deref().unwrap_or(""),
                "tef_parcelas": p.tef_parcelas.to_string(),
                "tef_tipo_parcelamento": p.tef_tipo_parcelamento.as_deref().unwrap_or(""),
                "tef_tipo_transacao": p.tef_tipo_transacao.map(|t| t.to_string()).unwrap_or_default(),
                "tef_desconto": p.tef_desconto.map(|d| format!("{:.2}", d).replace('.', ",")).unwrap_or_default(),
                "tef_saque": p.tef_saque.map(|s| format!("{:.2}", s).replace('.', ",")).unwrap_or_default(),
                "tef_sitef_instituicao": p.tef_sitef_instituicao.as_deref().unwrap_or(""),
                "tef_sitef_bandeira": p.tef_sitef_bandeira.as_deref().unwrap_or(""),
                "tef_via_estabelecimento": p.tef_via_estabelecimento.as_deref().unwrap_or(""),
                "tef_via_cliente": p.tef_via_cliente.as_deref().unwrap_or(""),
            })
        }).collect();

        // Convert linked abastecimentos to JSON list
        let linked_abast_json: Vec<serde_json::Value> = linked_abast.iter().map(|abast| {
            serde_json::json!({
                "id": abast.id.to_string(),
                "bico_id": abast.bico_id,
                "retorno": abast.retorno,
                "quantidade": format!("{:.2}", abast.quantidade).replace('.', ","),
                "valor_unitario": format!("{:.2}", abast.valor_unitario).replace('.', ","),
                "total": format!("{:.2}", abast.total).replace('.', ","),
                "tempo": abast.tempo.as_deref().unwrap_or("").trim(),
                "data": abast.data_hora.format("%d/%m/%Y").to_string(),
                "hora": abast.data_hora.format("%H:%M:%S").to_string(),
                "encerrante_inicial": format!("{:.2}", abast.encerrante_inicial).replace('.', ","),
                "encerrante_final": format!("{:.2}", abast.encerrante_final).replace('.', ","),
                "rfid_frentista": abast.rfid_frentista.as_deref().unwrap_or(""),
                "rfid_cliente": abast.rfid_cliente.as_deref().unwrap_or(""),
            })
        }).collect();

        let turno_id = if tipo_estabelecimento == "posto" {
            v.turno_posto_id
        } else {
            v.turno_id
        };
        let dados_adicionais = v
            .nfe_dados_adicionais
            .as_deref()
            .unwrap_or("")
            .replace("#$D#$A", "\n")
            .replace("#13#10", "\n");

        let nfe_xml_base64 = if let Some(ref xml) = v.nfe_xml {
            base64::Engine::encode(&base64::prelude::BASE64_STANDARD, xml.as_bytes())
        } else {
            String::new()
        };

        let payload = serde_json::json!({
            "tipo": "venda",
            "venda": [
                {
                    "empresa_id": empresa_id.to_string(),
                    "pdv_id": v.pdv.to_string(),
                    "setor_id": v.setor_id.to_string(),
                    "status": v.status,
                    "tipo": v.tipo.as_deref().unwrap_or(""),
                    "data": v.data_hora.map(|dt| dt.format("%d/%m/%Y").to_string()).unwrap_or_default(),
                    "hora": v.data_hora.map(|dt| dt.format("%H:%M:%S").to_string()).unwrap_or_default(),
                    "turno_id": turno_id.map(|uid| uid.to_string()).unwrap_or_default(),
                    "subtotal": format!("{:.2}", v.subtotal.unwrap_or_default()).replace('.', ","),
                    "desconto": format!("{:.2}", v.desconto.unwrap_or_default()).replace('.', ","),
                    "acrescimo": format!("{:.2}", v.acrescimo.unwrap_or_default()).replace('.', ","),
                    "valor_total": format!("{:.2}", v.valor_total.unwrap_or_default()).replace('.', ","),
                    "desconto_itens": format!("{:.2}", v.desconto_itens.unwrap_or_default()).replace('.', ","),
                    "acrescimo_itens": format!("{:.2}", v.acrescimo_itens.unwrap_or_default()).replace('.', ","),
                    "parceiro_id": v.parceiro_id.map(|p| p.to_string()).unwrap_or_default(),
                    "dependente_id": v.dependente_id.map(|d| d.to_string()).unwrap_or_default(),
                    "frota_id": v.frota_id.map(|f| f.to_string()).unwrap_or_default(),
                    "fidelidade_id": v.fidelidade_id.map(|f| f.to_string()).unwrap_or_default(),
                    "cpf_cnpj": v.cpf_cnpj.as_deref().unwrap_or(""),
                    "nome_fantasia": v.nome_fantasia.as_deref().unwrap_or(""),
                    "razao_social": v.razao_social.as_deref().unwrap_or(""),
                    "logradouro": v.logradouro.as_deref().unwrap_or(""),
                    "complemento": v.complemento.as_deref().unwrap_or(""),
                    "numero": v.numero.as_deref().unwrap_or(""),
                    "bairro": v.bairro.as_deref().unwrap_or(""),
                    "municipio": v.municipio.as_deref().unwrap_or(""),
                    "cod_municipio": v.cod_municipio.map(|c| c.to_string()).unwrap_or_default(),
                    "uf": v.uf.as_deref().unwrap_or(""),
                    "cep": v.cep.as_deref().unwrap_or(""),
                    "condutor": v.condutor.as_deref().unwrap_or(""),
                    "km": v.km.map(|k| k.to_string()).unwrap_or_default(),
                    "placa": v.placa.as_deref().unwrap_or(""),
                    "nfe_tipo": v.nfe_tipo.map(|t| t.to_string()).unwrap_or_default(),
                    "nfe_numero": v.nfe_numero.map(|n| n.to_string()).unwrap_or_default(),
                    "nfe_serie": v.nfe_serie.map(|s| s.to_string()).unwrap_or_default(),
                    "nfe_chave": v.nfe_chave.as_deref().unwrap_or(""),
                    "nfe_protocolo": v.nfe_protocolo.as_deref().unwrap_or(""),
                    "nfe_dados_adicionais": dados_adicionais,
                    "nfe_xml": nfe_xml_base64,
                    "usuario_pre_venda_id": v.usuario_pre_venda_id.map(|u| u.to_string()).unwrap_or_default(),
                    "pre_venda_id": v.pre_venda_id.map(|uid| uid.to_string()).unwrap_or_default(),
                    "pre_venda_number": v.pre_venda_numero.as_deref().unwrap_or(""),
                    "tabelapreco_id": v.tabela_preco_id.map(|t| t.to_string()).unwrap_or_default(),
                    "usuario_id": v.usuario_id.map(|u| u.to_string()).unwrap_or_default(),
                    "vendedor_id": v.vendedor_id.map(|vend| vend.to_string()).unwrap_or_default(),
                    "itens": items_json,
                    "abastecimentos": linked_abast_json,
                    "pagamentos": payments_json,
                    "cheques": Vec::<serde_json::Value>::new(),
                    "cheque_troco": Vec::<serde_json::Value>::new(),
                    "deposito_troco": Vec::<serde_json::Value>::new(),
                }
            ]
        });

        let url_with_query = sync_send_url(backend_url, "venda");
        info!(
            "EnvioDadosVenda - Enviando venda {} para {}",
            id, url_with_query
        );
        match http
            .post_json_servidor(&url_with_query, &payload.to_string(), token)
            .await
        {
            Ok(response_body) => {
                info!("EnvioDadosVenda - Resposta HTTP recebida para venda {}", id);
                if let Ok(result) = serde_json::from_str::<SincronizacaoResponse>(&response_body) {
                    if result.venda.and_then(|r| r.result).as_deref() == Some("success") {
                        let res = venda::Entity::update_many()
                            .col_expr(
                                venda::Column::Sincronizado,
                                sea_orm::sea_query::Expr::value("T"),
                            )
                            .filter(venda::Column::Id.eq(id))
                            .exec(db)
                            .await;
                        if let Err(e) = res {
                            error!(
                                "EnvioDadosVenda - Erro ao atualizar status de sincronização da venda: {:?}",
                                e
                            );
                        } else {
                            info!("EnvioDadosVenda - Venda {} sincronizada com sucesso.", id);
                        }
                    }
                }
            }
            Err(e) => {
                error!(
                    "EnvioDadosVenda - Erro na requisição HTTP para a venda {}: {:?}",
                    id, e
                );
            }
        }
    }

    Ok(())
}
