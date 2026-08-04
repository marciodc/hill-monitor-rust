use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect};
use hill_common::entity::afericao;
use hill_common::net::HttpConn;
use tracing::{error, info};

#[derive(serde::Deserialize)]
struct SincronizacaoResponse {
    afericao: Option<SincronizacaoResult>,
}

#[derive(serde::Deserialize)]
struct SincronizacaoResult {
    result: Option<String>,
}

pub async fn envia_afericoes(
    db: &DatabaseConnection,
    http: &HttpConn,
    backend_url: &str,
    token: &str,
    empresa_id: i32,
) -> Result<(), sea_orm::DbErr> {
    let afericoes = match afericao::Entity::find()
        .filter(afericao::Column::Sincronizado.ne("T").or(afericao::Column::Sincronizado.is_null()))
        .order_by_asc(afericao::Column::Id)
        .limit(50)
        .all(db)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            error!("EnvioDadosAfericao - Erro ao buscar aferições não sincronizadas: {:?}", e);
            return Err(e);
        }
    };

    if afericoes.is_empty() {
        return Ok(());
    }

    for afer in afericoes {
        let id = afer.id;
        let payload = serde_json::json!({
            "tipo": "afericao",
            "afericoes": [
                {
                    "empresa": empresa_id,
                    "setor_id": afer.setor_id,
                    "data": afer.data_hora.format("%d/%m/%Y").to_string(),
                    "hora": afer.data_hora.format("%H:%M:%S").to_string(),
                    "bico_id": afer.bico_id,
                    "abastecimento_id": afer.abastecimento_id.to_string(),
                    "quantidade": format!("{:.2}", afer.quantidade).replace('.', ","),
                    "turno_id": afer.turno_posto_id.to_string(),
                    "usuario_id": afer.usuario_id
                }
            ]
        });

        let url_with_query = format!("{}&tipo=afericao", backend_url);
        match http.post_json_servidor(&url_with_query, &payload.to_string(), token).await {
            Ok(response_body) => {
                if let Ok(result) = serde_json::from_str::<SincronizacaoResponse>(&response_body) {
                    if result.afericao.and_then(|r| r.result).as_deref() == Some("success") {
                        let res = afericao::Entity::update_many()
                            .col_expr(afericao::Column::Sincronizado, sea_orm::sea_query::Expr::value("T"))
                            .filter(afericao::Column::Id.eq(id))
                            .exec(db)
                            .await;
                        if let Err(e) = res {
                            error!("EnvioDadosAfericao - Erro ao atualizar status de sincronização da aferição: {:?}", e);
                        } else {
                            info!("EnvioDadosAfericao - Aferição {} sincronizada com sucesso.", id);
                        }
                    }
                }
            }
            Err(e) => {
                error!("EnvioDadosAfericao - Erro na requisição HTTP para a aferição {}: {:?}", id, e);
            }
        }
    }

    Ok(())
}
