use crate::backend_url::api_base_url;
use chrono::SecondsFormat;
use hill_common::entity::abastecimento;
use hill_common::net::HttpConn;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use tracing::{error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct AbastecimentosPushResponse {
    resultados: Option<Vec<AbastecimentosPushResultado>>,
}

#[derive(serde::Deserialize)]
struct AbastecimentosPushResultado {
    uuid_externo: String,
    ok: bool,
}

fn map_status(status: Option<&str>) -> &'static str {
    match status.unwrap_or_default().to_ascii_lowercase().as_str() {
        "a" | "aberto" => "aberto",
        _ => "finalizado",
    }
}

pub async fn envia_abastecimentos(
    db: &DatabaseConnection,
    http: &HttpConn,
    backend_url: &str,
    token: &str,
    empresa_id: i32,
) -> Result<(), sea_orm::DbErr> {
    let abastecimentos = match abastecimento::Entity::find()
        .filter(
            abastecimento::Column::Sincronizado
                .ne("T")
                .or(abastecimento::Column::Sincronizado.is_null()),
        )
        .order_by_asc(abastecimento::Column::Id)
        .limit(100)
        .all(db)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            error!(
                "EnvioDadosAbastecimento - Erro ao buscar abastecimentos não sincronizados: {:?}",
                e
            );
            return Err(e);
        }
    };

    if abastecimentos.is_empty() {
        return Ok(());
    }

    info!(
        "EnvioDadosAbastecimento - Encontrados {} abastecimentos para sincronizar.",
        abastecimentos.len()
    );

    let itens: Vec<serde_json::Value> = abastecimentos
        .iter()
        .map(|abast| {
            serde_json::json!({
                "uuid_externo": abast.id.to_string(),
                "bico_id": abast.bico_id,
                "litros": abast.quantidade.to_f64().unwrap_or(0.0),
                "valor_unitario": abast.valor_unitario.to_f64().unwrap_or(0.0),
                "valor_total": abast.total.to_f64().unwrap_or(0.0),
                "data_hora": abast.data_hora.and_utc().to_rfc3339_opts(SecondsFormat::Millis, true),
                "status": map_status(abast.status.as_deref()),
                "encerrante_inicial": abast.encerrante_inicial.to_f64().unwrap_or(0.0),
                "encerrante_final": abast.encerrante_final.to_f64().unwrap_or(0.0),
                "cliente_rfid": abast.rfid_cliente,
            })
        })
        .collect();

    let payload = serde_json::json!({
        "empresa_id": empresa_id,
        "itens": itens,
    });

    let url = format!("{}/pdv/abastecimentos", api_base_url(backend_url));
    info!(
        "EnvioDadosAbastecimento - Enviando lote de {} abastecimentos para {}",
        abastecimentos.len(),
        url
    );

    match http.post_json_servidor(&url, &payload.to_string(), token).await {
        Ok(response_body) => {
            let mut ids_sucesso: Vec<Uuid> = Vec::new();

            if let Ok(result) = serde_json::from_str::<AbastecimentosPushResponse>(&response_body)
            {
                if let Some(resultados) = result.resultados {
                    for r in resultados {
                        if r.ok {
                            if let Ok(id) = Uuid::parse_str(&r.uuid_externo) {
                                ids_sucesso.push(id);
                            }
                        }
                    }
                }
            } else {
                error!(
                    "EnvioDadosAbastecimento - Resposta fora do contrato esperado de /api/pdv/abastecimentos: {}",
                    response_body
                );
            }

            if ids_sucesso.is_empty() {
                info!(
                    "EnvioDadosAbastecimento - Nenhum abastecimento confirmado como sincronizado neste lote."
                );
                return Ok(());
            }

            let total_sucesso = ids_sucesso.len();
            let res = abastecimento::Entity::update_many()
                .col_expr(
                    abastecimento::Column::Sincronizado,
                    sea_orm::sea_query::Expr::value("T"),
                )
                .filter(abastecimento::Column::Id.is_in(ids_sucesso))
                .exec(db)
                .await;

            if let Err(e) = res {
                error!(
                    "EnvioDadosAbastecimento - Erro ao atualizar status de sincronização no banco: {:?}",
                    e
                );
            } else {
                info!(
                    "EnvioDadosAbastecimento - {} abastecimentos sincronizados com sucesso.",
                    total_sucesso
                );
            }
        }
        Err(e) => {
            error!(
                "EnvioDadosAbastecimento - Erro na requisição HTTP do lote de abastecimentos: {:?}",
                e
            );
        }
    }

    Ok(())
}
