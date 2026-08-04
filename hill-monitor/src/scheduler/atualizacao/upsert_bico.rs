use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::bico;

pub async fn upsert_bicos(db: &DatabaseConnection, bicos: &[hill_common::entity::Bico]) -> Result<(), DbErr> {
    if bicos.is_empty() {
        return Ok(());
    }

    let active_models: Vec<bico::ActiveModel> = bicos.iter().map(|bico| {
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
    }).collect();

    use sea_orm::{Iterable, IdenStatic};
    bico::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(bico::Column::Id)
                .update_columns(bico::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}
