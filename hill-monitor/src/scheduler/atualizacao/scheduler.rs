use crate::scheduler::atualizacao::upsert_bico::upsert_bicos;
use crate::scheduler::atualizacao::upsert_configuracao::upsert_configuracoes;
use crate::scheduler::atualizacao::upsert_usuario::upsert_usuarios;
use crate::scheduler::atualizacao::upsert_produto::upsert_produtos;
use crate::scheduler::atualizacao::upsert_forma_pagamento::upsert_formas_pagamento;
use crate::scheduler::atualizacao::upsert_parceiro::upsert_parceiros;
use crate::scheduler::atualizacao::upsert_administradora::upsert_administradoras;
use crate::scheduler::atualizacao::upsert_parceiro_dependente::upsert_parceiro_dependentes;
use crate::scheduler::atualizacao::upsert_parceiro_frota::upsert_parceiro_frotas;
use crate::scheduler::atualizacao::upsert_parceiro_forma_pagamento::upsert_parceiro_formas_pagamento;
use crate::scheduler::atualizacao::upsert_parceiro_tabela_forma_pagamento::upsert_parceiro_tabelas_formas_pagamento;
use crate::scheduler::atualizacao::upsert_parceiro_tabela::upsert_parceiro_tabelas;
use crate::scheduler::atualizacao::upsert_setor::upsert_setores;
use crate::scheduler::atualizacao::upsert_produto_setor::upsert_produtos_setores;
use crate::scheduler::atualizacao::upsert_tanque::upsert_tanques;
use crate::scheduler::atualizacao::upsert_tabela_preco::upsert_tabela_precos;
use crate::scheduler::atualizacao::upsert_tabela_preco_item::upsert_tabela_preco_itens;
use crate::scheduler::atualizacao::upsert_usuario_permissao::upsert_usuario_permissoes;
use crate::scheduler::atualizacao::upsert_vendedor::upsert_vendedores;

use hill_common::config_helper::ConfigHelper;
use hill_common::net::HttpConn;
use serde::Deserialize;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SincronizacaoPayload {
    pub sincronizacao: Option<Sincronizacao>,
}

#[derive(Deserialize)]
pub struct Sincronizacao {
    pub result: Option<String>,
    pub message: Option<String>,
    pub bicos: Option<Vec<hill_common::entity::Bico>>,
    pub configuracoes: Option<Vec<hill_common::entity::Configuracao>>,
    pub usuarios: Option<Vec<hill_common::entity::Usuario>>,
    pub produtos: Option<Vec<hill_common::entity::Produto>>,
    pub moedas: Option<Vec<hill_common::entity::FormaPagamento>>,
    pub parceiros: Option<Vec<hill_common::entity::Parceiro>>,
    pub administradoras: Option<Vec<hill_common::entity::Administradora>>,
    pub parceiro_dependentes: Option<Vec<hill_common::entity::ParceiroDependente>>,
    pub parceiro_frotas: Option<Vec<hill_common::entity::ParceiroFrota>>,
    #[serde(rename = "parceiro_moedas")]
    pub parceiro_formas_pagamento: Option<Vec<hill_common::entity::ParceiroFormaPagamento>>,
    #[serde(rename = "parceiro_tabela_moedas")]
    pub parceiro_tabelas_formas_pagamento: Option<Vec<hill_common::entity::ParceiroTabelaFormaPagamento>>,
    pub parceiro_tabelas: Option<Vec<hill_common::entity::ParceiroTabela>>,
    pub setores: Option<Vec<hill_common::entity::Setor>>,
    pub produtos_setores: Option<Vec<hill_common::entity::ProdutoSetor>>,
    pub tanques: Option<Vec<hill_common::entity::Tanque>>,
    #[serde(rename = "tabelasprecos")]
    pub tabela_precos: Option<Vec<hill_common::entity::TabelaPreco>>,
    pub tabelapreco_itens: Option<Vec<hill_common::entity::TabelaPrecoItem>>,
    pub usuario_permissoes: Option<Vec<hill_common::entity::UsuarioPermissao>>,
    pub vendedores: Option<Vec<hill_common::entity::Vendedor>>,
}

pub struct AtualizacaoScheduler {
    db: DatabaseConnection,
    pdv_uuid: Uuid,
    running: Arc<AtomicBool>,
}

impl AtualizacaoScheduler {
    pub fn new(db: DatabaseConnection, pdv_uuid: Uuid) -> Self {
        Self {
            db,
            pdv_uuid,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let running = self.running.clone();
        let pdv_uuid = self.pdv_uuid;

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Roda a cada 1 minuto
            let config_helper = ConfigHelper::new(db.clone());
            let http = HttpConn::new();

            info!("AtualizacaoScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;
                info!("AtualizacaoScheduler executando sincronização de dados para o PDV: {}", pdv_uuid);

                // Load URL and Token parameters dynamically from DB
                let backend_url = match config_helper.get_parametro("PDV_BackendURL", Some(pdv_uuid)).await {
                    Ok(Some(url)) if !url.is_empty() => url,
                    _ => {
                        info!("PDV_BackendURL não configurada no banco de dados. Pulando sincronização.");
                        continue;
                    }
                };

                let token = match config_helper.get_parametro("PDV_Token", Some(pdv_uuid)).await {
                    Ok(Some(t)) if !t.is_empty() => t,
                    _ => {
                        info!("PDV_Token não configurado no banco de dados. Pulando sincronização.");
                        continue;
                    }
                };

                // Load last synchronization date from DB configuracoes table
                use hill_common::entity::configuracao;
                let last_sinc = match configuracao::Entity::find_by_id(pdv_uuid).one(&db).await {
                    Ok(Some(conf)) => conf.atualizacao,
                    _ => None,
                };

                let data_sinc_str = match last_sinc {
                    Some(dt) => dt.format("%d/%m/%Y %H:%M:%S").to_string(),
                    None => "01/01/1900 00:00:00".to_string(),
                };

                // Create JSON payload
                let payload = serde_json::json!({
                    "tipo": "update",
                    "sincronizacao": [
                        {
                            "pdv_id": pdv_uuid.to_string(),
                            "data": data_sinc_str,
                            "requisicao": "pdv",
                            "tipo": "update"
                        }
                    ]
                });

                let url_with_query = format!("{}&tipo=update", backend_url);
                info!("Sincronizacao - Enviando dados para: {}", url_with_query);

                match http.post_json_servidor(&url_with_query, &payload.to_string(), &token).await {
                    Ok(response_body) => {
                        info!("Sincronizacao - Retorno recebido com sucesso.");
                        
                        match serde_json::from_str::<SincronizacaoPayload>(&response_body) {
                            Ok(parsed) => {
                                if let Some(sinc) = parsed.sincronizacao {
                                    if sinc.result.as_deref() == Some("success") {
                                        let mut success = false;
                                        
                                        // 1. Upsert Configuracoes
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

                                        // 2. Upsert Bicos
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

                                        // 3. Upsert Usuarios
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

                                        // 4. Upsert Produtos
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

                                        // 5. Upsert Formas de Pagamento
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

                                        // 6. Upsert Parceiros
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

                                        // 7. Upsert Administradoras
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

                                        // 8. Upsert Setores
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

                                        // 9. Upsert Tanques
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

                                        // 10. Upsert Tabela Precos
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

                                        // 11. Upsert Vendedores
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

                                        // 12. Upsert Usuario Permissoes
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

                                        // 13. Upsert Tabela Preco Itens
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

                                        // 14. Upsert Produtos Setores
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

                                        // 15. Upsert Parceiro Dependentes
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

                                        // 16. Upsert Parceiro Frotas
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

                                        // 17. Upsert Parceiro Formas Pagamento
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

                                        // 18. Upsert Parceiro Tabelas Formas Pagamento
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

                                        // 19. Upsert Parceiro Tabelas
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
                                            // Update synchronization timestamp
                                            let now = chrono::Utc::now().naive_utc();
                                            use hill_common::entity::configuracao;
                                            use sea_orm::ColumnTrait;
                                            let res = configuracao::Entity::update_many()
                                                .col_expr(configuracao::Column::Atualizacao, sea_orm::sea_query::Expr::value(now))
                                                .filter(configuracao::Column::Id.eq(pdv_uuid))
                                                .exec(&db)
                                                .await;
                                            if let Err(e) = res {
                                                error!("Erro ao atualizar timestamp de sincronização: {:?}", e);
                                            } else {
                                                info!("Sincronização do PDV concluída com sucesso.");
                                            }
                                        }
                                    } else {
                                        error!("Sincronização retornou falha: {:?}", sinc.message);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Erro ao fazer parse da resposta de sincronização: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Sincronizacao - Erro na requisição HTTP: {:?}", e);
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
