use hill_common::entity::bico;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_bicos(
    db: &DatabaseConnection,
    bicos: &[hill_common::entity::Bico],
) -> Result<(), DbErr> {
    if bicos.is_empty() {
        return Ok(());
    }

    for bico in bicos {
        if let Some(existing) = bico::Entity::find_by_id(bico.id).one(db).await? {
            let existing: bico::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        bico::ActiveModel {
            id: ActiveValue::Set(bico.id),
            status: ActiveValue::Set(bico.status.clone()),
            retorno: ActiveValue::Set(bico.retorno.clone()),
            numero: ActiveValue::Set(bico.numero),
            bomba: ActiveValue::Set(bico.bomba),
            tanque_id: ActiveValue::Set(bico.tanque_id),
            bloqueio_quantidade: ActiveValue::Set(bico.bloqueio_quantidade),
            setor_id: ActiveValue::Set(bico.setor_id),
            produto_id: ActiveValue::Set(bico.produto_id),
            gtin: ActiveValue::Set(bico.gtin.clone()),
            valor_unitario: ActiveValue::Set(bico.valor_unitario),
            valor_unitario_debito: ActiveValue::Set(bico.valor_unitario_debito),
            valor_unitario_credito: ActiveValue::Set(bico.valor_unitario_credito),
            combustivel: ActiveValue::Set(bico.combustivel.clone()),
            altera_preco: ActiveValue::Set(bico.altera_preco.clone()),
            tabelapreco_id: ActiveValue::Set(bico.tabelapreco_id),
            tipo_combustivel: ActiveValue::Set(bico.tipo_combustivel.clone()),
            abastecimento_manual: ActiveValue::Set(bico.abastecimento_manual.clone()),
            bloqueado: ActiveValue::Set(bico.bloqueado.clone()),
            sincroniza_preco_data_hora: ActiveValue::Set(bico.sincroniza_preco_data_hora),
            sincroniza_preco_alterado: ActiveValue::Set(bico.sincroniza_preco_alterado.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
