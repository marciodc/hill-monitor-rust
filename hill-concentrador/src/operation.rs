use crate::com::ConcentradorCom;
use crate::companytec::Companytec;
use chrono::Utc;
use hill_common::entity::{StatusBico, abastecimento, bico};
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, PaginatorTrait,
    QueryFilter, Statement,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tracing::{error, info};

#[derive(Clone)]
pub struct ConcentradorOperacao {
    com: ConcentradorCom,
    equipamento: String,
}

impl ConcentradorOperacao {
    pub fn new(com: ConcentradorCom, fabricante: &str) -> Self {
        Self {
            com,
            equipamento: fabricante.to_lowercase(),
        }
    }

    pub async fn status_bicos(&self, db: &DatabaseConnection) -> Vec<StatusBico> {
        let mut active_retornos = Vec::new();
        if self.equipamento == "companytec" {
            let ret = Companytec::visualiza_abastecimento(&self.com).await;
            if !ret.is_empty() {
                // Regex-like extraction of Groups of 2 chars every 8 chars
                let chars = ret.chars().collect::<Vec<char>>();
                let mut i = 0;
                while i + 2 <= chars.len() {
                    let bico: String = chars[i..i + 2].iter().collect();
                    active_retornos.push(bico);
                    i += 8;
                }
            }
        }

        // Fetch bicos and status from Database using SeaORM raw query
        let query = r#"
            SELECT 
                b.id, 
                b.retorno, 
                b.numero, 
                b.bloqueado,
                EXISTS(SELECT 1 FROM abastecimentos a WHERE a.retorno = b.retorno AND a.status = 'P') as has_pending,
                (SELECT COUNT(*) FROM abastecimentos a WHERE a.retorno = b.retorno AND a.status = 'P') as pending_count
            FROM bicos b
            ORDER BY b.numero
        "#;

        let statement = Statement::from_string(DbBackend::Postgres, query.to_string());
        let rows = match db.query_all_raw(statement).await {
            Ok(r) => r,
            Err(e) => {
                error!("Erro ao consultar bicos do banco de dados: {:?}", e);
                return Vec::new();
            }
        };

        let mut bicos = Vec::new();
        for row in rows {
            let id: i32 = row.try_get("", "id").unwrap_or_default();
            let retorno: String = row
                .try_get::<Option<String>>("", "retorno")
                .unwrap_or_default()
                .unwrap_or_default();
            let numero: i32 = row.try_get("", "numero").unwrap_or_default();
            let bloqueado_str: String = row
                .try_get::<Option<String>>("", "bloqueado")
                .unwrap_or_default()
                .unwrap_or_default();
            let has_pending: bool = row.try_get("", "has_pending").unwrap_or_default();
            let pending_count: i64 = row.try_get("", "pending_count").unwrap_or_default();

            let is_active = active_retornos.contains(&retorno);
            let status = if is_active {
                "A".to_string()
            } else if has_pending {
                "P".to_string()
            } else {
                "F".to_string()
            };

            bicos.push(StatusBico {
                id,
                retorno: retorno.clone(),
                numero,
                bloqueado: bloqueado_str == "T",
                status: Some(status),
                quantidade: pending_count as i32,
            });
        }

        bicos
    }

    pub async fn consulta_encerrante(&self, bico: &str, decimais: i32) -> Decimal {
        let mut tentativa = 1;
        let mut encerrante = Decimal::ZERO;

        while tentativa <= 3 {
            if self.equipamento == "companytec" {
                encerrante = Companytec::consulta_encerrante(&self.com, bico, decimais).await;
            }

            if encerrante > Decimal::ZERO {
                break;
            }

            tentativa += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        encerrante
    }

    pub async fn bloqueia_bico(&self, bico: &str) -> bool {
        if self.equipamento == "companytec" {
            Companytec::bloqueia_bico(&self.com, bico).await
        } else {
            false
        }
    }

    pub async fn desbloqueia_bico(&self, bico: &str) -> bool {
        if self.equipamento == "companytec" {
            Companytec::desbloqueia_bico(&self.com, bico).await
        } else {
            false
        }
    }

    pub async fn atualiza_preco_vista(&self, bico: &str, valor: Decimal) -> bool {
        if self.equipamento == "companytec" {
            Companytec::atualiza_preco(&self.com, bico, 0, valor).await
        } else {
            false
        }
    }

    pub async fn atualiza_preco_banco(&self, db: &DatabaseConnection) {
        let rows = match bico::Entity::find()
            .filter(bico::Column::AlteraPreco.eq("T"))
            .all(db)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Erro ao consultar bicos para atualizar preços: {:?}", e);
                return;
            }
        };

        for b in rows {
            let retorno = b.retorno.clone().unwrap_or_default();
            if self.atualiza_preco_vista(&retorno, b.valor_unitario).await {
                use sea_orm::ActiveModelTrait;
                let mut active_bico: bico::ActiveModel = b.into();
                active_bico.altera_preco = sea_orm::ActiveValue::Set(Some("F".to_string()));
                active_bico.sincroniza_preco_data_hora =
                    sea_orm::ActiveValue::Set(Some(Utc::now().naive_utc()));
                active_bico.sincroniza_preco_alterado =
                    sea_orm::ActiveValue::Set(Some("T".to_string()));

                if let Err(e) = active_bico.update(db).await {
                    error!("Erro ao atualizar status do bico no banco: {:?}", e);
                } else {
                    info!(
                        "Preço do bico {} atualizado com sucesso no banco de dados.",
                        retorno
                    );
                }
            }
        }
    }

    pub async fn move_ponteiro(&self) {
        if self.equipamento == "companytec" {
            Companytec::incremento(&self.com).await;
        }
    }

    pub async fn captura_abastecimento(&self, db: &DatabaseConnection) {
        let mut abastecimento = None;
        if self.equipamento == "companytec" {
            abastecimento = Companytec::captura_abastecimento(&self.com).await;
        }

        if let Some(mut abast) = abastecimento {
            if abast.valor_unitario > Decimal::ZERO {
                let full_string = abast.full_string.clone().unwrap_or_default();
                Self::adicionar_abastecimento_arquivo(&full_string);

                if abast.quantidade < Decimal::new(1, 2) {
                    // 0.01
                    self.move_ponteiro().await;
                    return;
                }

                // Check duplicates in DB
                let count = match abastecimento::Entity::find()
                    .filter(abastecimento::Column::FullString.eq(full_string.clone()))
                    .count(db)
                    .await
                {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Erro ao checar duplicidade de abastecimento: {:?}", e);
                        return;
                    }
                };

                if count > 0 {
                    self.move_ponteiro().await;
                    return;
                }

                // Resolve bico_id
                let bico_retorno = abast.retorno.clone().unwrap_or_default();
                let bico_numero: i32 = match bico::Entity::find()
                    .filter(bico::Column::Retorno.eq(bico_retorno))
                    .one(db)
                    .await
                {
                    Ok(Some(b)) => b.numero,
                    _ => 0,
                };

                abast.bico_id = bico_numero;

                // Insert into DB
                use sea_orm::ActiveModelTrait;
                let active_abast = abastecimento::ActiveModel::from(abast);

                if let Err(e) = active_abast.insert(db).await {
                    error!("Erro ao inserir abastecimento no banco de dados: {:?}", e);
                } else {
                    info!(
                        "Abastecimento capturado e inserido no banco: Bico {}",
                        bico_numero
                    );
                    self.move_ponteiro().await;
                }
            }
        }
    }

    fn adicionar_abastecimento_arquivo(abastecimento_line: &str) {
        let current_time = chrono::Local::now();
        let log_dir = Path::new("log");
        let _ = std::fs::create_dir_all(log_dir);

        let file_name = log_dir.join(format!(
            "abastecimentos-{}.log",
            current_time.format("%Y-%m-%d")
        ));
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(file_name) {
            let _ = writeln!(file, "{}", abastecimento_line);
        }
    }
}
