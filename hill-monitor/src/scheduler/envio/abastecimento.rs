use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect};
use hill_common::entity::abastecimento;
use hill_common::net::HttpConn;
use tracing::{error, info};

#[derive(serde::Deserialize)]
struct SincronizacaoResponse {
    abastecimentos: Option<SincronizacaoResult>,
}

#[derive(serde::Deserialize)]
struct SincronizacaoResult {
    result: Option<String>,
}

pub async fn envia_abastecimentos(
    db: &DatabaseConnection,
    http: &HttpConn,
    backend_url: &str,
    token: &str,
    empresa_id: i32,
) -> Result<(), sea_orm::DbErr> {
    let abastecimentos = match abastecimento::Entity::find()
        .filter(abastecimento::Column::Sincronizado.ne("T").or(abastecimento::Column::Sincronizado.is_null()))
        .order_by_asc(abastecimento::Column::Id)
        .limit(50)
        .all(db)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            error!("EnvioDadosAbastecimento - Erro ao buscar abastecimentos não sincronizados: {:?}", e);
            return Err(e);
        }
    };

    if abastecimentos.is_empty() {
        return Ok(());
    }

    info!("EnvioDadosAbastecimento - Encontrados {} abastecimentos para sincronizar.", abastecimentos.len());

    for abast in abastecimentos {
        let id = abast.id;
        let payload = serde_json::json!({
            "tipo": "abastecimentos",
            "abastecimentos": [
                {
                    "id": id.to_string(),
                    "empresa": empresa_id,
                    "bico_id": abast.bico_id,
                    "retorno": abast.retorno,
                    "quantidade": format!("{:.2}", abast.quantidade).replace('.', ","),
                    "valor_unitario": format!("{:.2}", abast.valor_unitario).replace('.', ","),
                    "total": format!("{:.2}", abast.total).replace('.', ","),
                    "tempo": abast.tempo.as_deref().unwrap_or("").trim(),
                    "data_hora": abast.data_hora.format("%d/%m/%Y %H:%M:%S").to_string(),
                    "encerrante_inicial": format!("{:.2}", abast.encerrante_inicial).replace('.', ","),
                    "encerrante_final": format!("{:.2}", abast.encerrante_final).replace('.', ","),
                    "rfid_frentista": abast.rfid_frentista,
                    "rfid_cliente": abast.rfid_cliente,
                    "gerado": abast.gerado,
                    "desmembramento_id": abast.desmembramento_id.map(|uid| uid.to_string()),
                    "full_string": abast.full_string,
                }
            ]
        });

        let url_with_query = format!("{}&tipo=abastecimentos", backend_url);
        match http.post_json_servidor(&url_with_query, &payload.to_string(), token).await {
            Ok(response_body) => {
                if let Ok(result) = serde_json::from_str::<SincronizacaoResponse>(&response_body) {
                    if result.abastecimentos.and_then(|r| r.result).as_deref() == Some("success") {
                        let res = abastecimento::Entity::update_many()
                            .col_expr(abastecimento::Column::Sincronizado, sea_orm::sea_query::Expr::value("T"))
                            .filter(abastecimento::Column::Id.eq(id))
                            .exec(db)
                            .await;
                        if let Err(e) = res {
                            error!("EnvioDadosAbastecimento - Erro ao atualizar status de sincronização no banco: {:?}", e);
                        } else {
                            info!("EnvioDadosAbastecimento - Abastecimento {} sincronizado com sucesso.", id);
                        }
                    }
                }
            }
            Err(e) => {
                error!("EnvioDadosAbastecimento - Erro na requisição HTTP para o abastecimento {}: {:?}", id, e);
            }
        }
    }

    Ok(())
}
