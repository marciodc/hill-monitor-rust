use crate::scheduler::atualizacao::upsert_administradora::upsert_administradoras;
use crate::scheduler::atualizacao::upsert_bico::upsert_bicos;
use crate::scheduler::atualizacao::upsert_configuracao::upsert_configuracoes;
use crate::scheduler::atualizacao::upsert_forma_pagamento::upsert_formas_pagamento;
use crate::scheduler::atualizacao::upsert_parceiro::upsert_parceiros;
use crate::scheduler::atualizacao::upsert_parceiro_dependente::upsert_parceiro_dependentes;
use crate::scheduler::atualizacao::upsert_parceiro_forma_pagamento::upsert_parceiro_formas_pagamento;
use crate::scheduler::atualizacao::upsert_parceiro_frota::upsert_parceiro_frotas;
use crate::scheduler::atualizacao::upsert_parceiro_tabela::upsert_parceiro_tabelas;
use crate::scheduler::atualizacao::upsert_parceiro_tabela_forma_pagamento::upsert_parceiro_tabelas_formas_pagamento;
use crate::scheduler::atualizacao::upsert_produto::upsert_produtos;
use crate::scheduler::atualizacao::upsert_produto_setor::upsert_produtos_setores;
use crate::scheduler::atualizacao::upsert_setor::upsert_setores;
use crate::scheduler::atualizacao::upsert_tabela_preco::upsert_tabela_precos;
use crate::scheduler::atualizacao::upsert_tabela_preco_item::upsert_tabela_preco_itens;
use crate::scheduler::atualizacao::upsert_tanque::upsert_tanques;
use crate::scheduler::atualizacao::upsert_usuario::upsert_usuarios;
use crate::scheduler::atualizacao::upsert_usuario_permissao::upsert_usuario_permissoes;
use crate::scheduler::atualizacao::upsert_vendedor::upsert_vendedores;
use crate::backend_url::api_base_url;
use hill_common::config_helper::ConfigHelper;
use hill_common::net::HttpConn;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info};

use super::dto::*;
use super::mapper::{map_new_payload_to_sincronizacao, parse_datetime};

#[derive(Deserialize)]
struct HeartbeatEnvelope {
    result: Option<HeartbeatResult>,
}

#[derive(Deserialize)]
struct HeartbeatResult {
    data: Option<HeartbeatData>,
}

#[derive(Deserialize)]
struct HeartbeatData {
    json: Option<HeartbeatJson>,
}

#[derive(Deserialize)]
struct HeartbeatJson {
    ok: bool,
    tem_atualizacao: Option<bool>,
}

fn snapshot_url(base_url: &str) -> String {
    format!("{}/pdv/sync/snapshot", base_url)
}

fn delta_url(base_url: &str) -> String {
    format!("{}/pdv/sync/delta", base_url)
}

fn heartbeat_url(base_url: &str) -> String {
    format!("{}/trpc/pdvSync.heartbeat", base_url)
}

fn response_preview(body: &str, max_len: usize) -> String {
    body.trim().chars().take(max_len).collect()
}

pub struct AtualizacaoScheduler {
    db: DatabaseConnection,
    running: Arc<AtomicBool>,
}

impl AtualizacaoScheduler {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let running = self.running.clone();

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            let config_helper = ConfigHelper::new(db.clone());
            let http = HttpConn::new();

            info!("AtualizacaoScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;

                let backend_url = match config_helper.get_parametro("Backend_URL", None).await {
                    Ok(Some(url)) if !url.is_empty() => url,
                    _ => {
                        info!("Backend_URL não configurada no banco de dados. Pulando sincronização.");
                        continue;
                    }
                };

                let token = match config_helper.get_parametro("Backend_Token", None).await {
                    Ok(Some(value)) if !value.is_empty() => value,
                    _ => {
                        info!("Backend_Token não configurado no banco de dados. Pulando sincronização.");
                        continue;
                    }
                };

                let configuracoes = match config_helper.list_configuracoes().await {
                    Ok(configs) => configs,
                    Err(e) => {
                        error!("Erro ao carregar PDVs para sincronização: {:?}", e);
                        continue;
                    }
                };

                let base_url = api_base_url(&backend_url);
                let bearer_token = format!("Bearer {}", token);

                let last_sinc = configuracoes.iter().filter_map(|c| c.atualizacao).max();

                let response_result = match last_sinc {
                    None => {
                        let url = snapshot_url(&base_url);
                        info!("Sincronizacao - Snapshot global: {}", url);
                        let response = http.get_json_servidor(&url, &bearer_token).await;
                        match &response {
                            Ok(_) => info!("Sincronizacao - Snapshot global HTTP concluído."),
                            Err(e) => error!("Sincronizacao - Snapshot global HTTP falhou: {:?}", e),
                        }
                        response
                    }
                    Some(dt) => {
                        let desde = dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                        let url = heartbeat_url(&base_url);
                        let payload = serde_json::json!({
                            "json": {
                                "app_version": env!("CARGO_PKG_VERSION"),
                                "desde": desde
                            }
                        });

                        info!("Sincronizacao - Heartbeat global a partir de {}: {}", desde, url);

                        let heartbeat_body = match http
                            .post_json_servidor(&url, &payload.to_string(), &bearer_token)
                            .await
                        {
                            Ok(body) => {
                                info!("Sincronizacao - Heartbeat global HTTP concluído.");
                                body
                            }
                            Err(e) => {
                                error!("Sincronizacao - Erro no heartbeat HTTP global: {:?}", e);
                                continue;
                            }
                        };

                        let heartbeat = match serde_json::from_str::<HeartbeatEnvelope>(&heartbeat_body) {
                            Ok(parsed) => parsed,
                            Err(e) => {
                                error!("Sincronizacao - Erro ao fazer parse do heartbeat global: {:?}", e);
                                continue;
                            }
                        };

                        let tem_atualizacao = heartbeat
                            .result
                            .and_then(|r| r.data)
                            .and_then(|d| d.json)
                            .map(|j| j.ok && j.tem_atualizacao.unwrap_or(false))
                            .unwrap_or(false);

                        if !tem_atualizacao {
                            info!("Sincronizacao - Heartbeat global sem atualizações pendentes.");
                            continue;
                        }

                        let url = delta_url(&base_url);
                        let payload = serde_json::json!({ "desde": desde });
                        info!("Sincronizacao - Delta global a partir de {}: {}", dt.format("%Y-%m-%dT%H:%M:%SZ"), url);
                        let response =
                            http.post_json_servidor(&url, &payload.to_string(), &bearer_token)
                                .await;
                        match &response {
                            Ok(_) => info!("Sincronizacao - Delta global HTTP concluído."),
                            Err(e) => error!("Sincronizacao - Delta global HTTP falhou: {:?}", e),
                        }
                        response
                    }
                };

                match response_result {
                        Ok(response_body) => {
                            info!("Sincronizacao - Retorno global recebido com sucesso.");

                            match serde_json::from_str::<NewSyncPayload>(&response_body) {
                                Ok(parsed) => {
                                    if !parsed.ok {
                                        error!("Sincronização global retornou ok=false");
                                        continue;
                                    }

                                    let gerado_em_dt = parsed
                                        .gerado_em
                                        .as_deref()
                                        .and_then(parse_datetime)
                                        .unwrap_or_else(|| chrono::Utc::now().naive_utc());

                                    let sinc = map_new_payload_to_sincronizacao(
                                        parsed,
                                        None,
                                    );
                                    let mut success = false;

                                    if let Some(configs) = sinc.configuracoes {
                                        if !configs.is_empty() {
                                            if let Err(e) = upsert_configuracoes(&db, &configs).await {
                                                error!("Erro ao atualizar configurações: {:?}", e);
                                            } else {
                                                info!("Configurações atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(bicos) = sinc.bicos {
                                        if !bicos.is_empty() {
                                            if let Err(e) = upsert_bicos(&db, &bicos).await {
                                                error!("Erro ao atualizar bicos: {:?}", e);
                                            } else {
                                                info!("Bicos atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(usuarios) = sinc.usuarios {
                                        if !usuarios.is_empty() {
                                            if let Err(e) = upsert_usuarios(&db, &usuarios).await {
                                                error!("Erro ao atualizar usuários: {:?}", e);
                                            } else {
                                                info!("Usuários atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(produtos) = sinc.produtos {
                                        if !produtos.is_empty() {
                                            if let Err(e) = upsert_produtos(&db, &produtos).await {
                                                error!("Erro ao atualizar produtos: {:?}", e);
                                            } else {
                                                info!("Produtos atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(formas) = sinc.moedas {
                                        if !formas.is_empty() {
                                            if let Err(e) = upsert_formas_pagamento(&db, &formas).await {
                                                error!("Erro ao atualizar formas de pagamento: {:?}", e);
                                            } else {
                                                info!("Formas de pagamento atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(parceiros) = sinc.parceiros {
                                        if !parceiros.is_empty() {
                                            if let Err(e) = upsert_parceiros(&db, &parceiros).await {
                                                error!("Erro ao atualizar parceiros: {:?}", e);
                                            } else {
                                                info!("Parceiros atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(admins) = sinc.administradoras {
                                        if !admins.is_empty() {
                                            if let Err(e) = upsert_administradoras(&db, &admins).await {
                                                error!("Erro ao atualizar administradoras: {:?}", e);
                                            } else {
                                                info!("Administradoras atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(setores) = sinc.setores {
                                        if !setores.is_empty() {
                                            if let Err(e) = upsert_setores(&db, &setores).await {
                                                error!("Erro ao atualizar setores: {:?}", e);
                                            } else {
                                                info!("Setores atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(tanques) = sinc.tanques {
                                        if !tanques.is_empty() {
                                            if let Err(e) = upsert_tanques(&db, &tanques).await {
                                                error!("Erro ao atualizar tanques: {:?}", e);
                                            } else {
                                                info!("Tanques atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(tabelas) = sinc.tabela_precos {
                                        if !tabelas.is_empty() {
                                            if let Err(e) = upsert_tabela_precos(&db, &tabelas).await {
                                                error!("Erro ao atualizar tabela de preços: {:?}", e);
                                            } else {
                                                info!("Tabela de preços atualizada no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(vendedores) = sinc.vendedores {
                                        if !vendedores.is_empty() {
                                            if let Err(e) = upsert_vendedores(&db, &vendedores).await {
                                                error!("Erro ao atualizar vendedores: {:?}", e);
                                            } else {
                                                info!("Vendedores atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(permissoes) = sinc.usuario_permissoes {
                                        if !permissoes.is_empty() {
                                            if let Err(e) = upsert_usuario_permissoes(&db, &permissoes).await {
                                                error!("Erro ao atualizar permissões do usuário: {:?}", e);
                                            } else {
                                                info!("Permissões do usuário atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(itens) = sinc.tabelapreco_itens {
                                        if !itens.is_empty() {
                                            if let Err(e) = upsert_tabela_preco_itens(&db, &itens).await {
                                                error!("Erro ao atualizar itens da tabela de preços: {:?}", e);
                                            } else {
                                                info!("Itens da tabela de preços atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(prod_setores) = sinc.produtos_setores {
                                        if !prod_setores.is_empty() {
                                            if let Err(e) = upsert_produtos_setores(&db, &prod_setores).await {
                                                error!("Erro ao atualizar produtos setores: {:?}", e);
                                            } else {
                                                info!("Produtos setores atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(deps) = sinc.parceiro_dependentes {
                                        if !deps.is_empty() {
                                            if let Err(e) = upsert_parceiro_dependentes(&db, &deps).await {
                                                error!("Erro ao atualizar dependentes de parceiros: {:?}", e);
                                            } else {
                                                info!("Dependentes de parceiros atualizados no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(frotas) = sinc.parceiro_frotas {
                                        if !frotas.is_empty() {
                                            if let Err(e) = upsert_parceiro_frotas(&db, &frotas).await {
                                                error!("Erro ao atualizar frotas de parceiros: {:?}", e);
                                            } else {
                                                info!("Frotas de parceiros atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(formas_pag) = sinc.parceiro_formas_pagamento {
                                        if !formas_pag.is_empty() {
                                            if let Err(e) = upsert_parceiro_formas_pagamento(&db, &formas_pag).await {
                                                error!("Erro ao atualizar formas de pagamento de parceiros: {:?}", e);
                                            } else {
                                                info!("Formas de pagamento de parceiros atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(tabelas_pag) = sinc.parceiro_tabelas_formas_pagamento {
                                        if !tabelas_pag.is_empty() {
                                            if let Err(e) = upsert_parceiro_tabelas_formas_pagamento(&db, &tabelas_pag).await {
                                                error!("Erro ao atualizar tabelas formas pagamento de parceiros: {:?}", e);
                                            } else {
                                                info!("Tabelas formas pagamento de parceiros atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if let Some(parc_tabelas) = sinc.parceiro_tabelas {
                                        if !parc_tabelas.is_empty() {
                                            if let Err(e) = upsert_parceiro_tabelas(&db, &parc_tabelas).await {
                                                error!("Erro ao atualizar tabelas de parceiros: {:?}", e);
                                            } else {
                                                info!("Tabelas de parceiros atualizadas no banco.");
                                                success = true;
                                            }
                                        }
                                    }

                                    if success {
                                        let res = hill_common::entity::configuracao::Entity::update_many()
                                            .col_expr(
                                                hill_common::entity::configuracao::Column::Atualizacao,
                                                sea_orm::sea_query::Expr::value(gerado_em_dt),
                                            )
                                            .exec(&db)
                                            .await;

                                        if let Err(e) = res {
                                            error!("Erro ao atualizar timestamp de sincronização: {:?}", e);
                                        } else {
                                            info!("Sincronização global concluída com sucesso.");
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "Erro ao fazer parse da resposta de sincronização global: {:?}. Prévia da resposta: {:?}",
                                        e,
                                        response_preview(&response_body, 300)
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            error!("Sincronizacao - Erro na requisição HTTP global: {:?}", e);
                        }
                }
            }

            info!("AtualizacaoScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
