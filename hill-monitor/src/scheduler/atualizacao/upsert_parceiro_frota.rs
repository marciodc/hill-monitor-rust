use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::parceiro_frota;

pub async fn upsert_parceiro_frotas(
    db: &DatabaseConnection,
    frotas: &[hill_common::entity::ParceiroFrota],
) -> Result<(), DbErr> {
    if frotas.is_empty() {
        return Ok(());
    }

    let active_models: Vec<parceiro_frota::ActiveModel> = frotas
        .iter()
        .map(|f| parceiro_frota::ActiveModel {
            id: ActiveValue::Set(f.id),
            status: ActiveValue::Set(f.status.clone()),
            parceiro_id: ActiveValue::Set(f.parceiro_id),
            veiculo: ActiveValue::Set(f.veiculo.clone()),
            placa: ActiveValue::Set(f.placa.clone()),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    parceiro_frota::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(parceiro_frota::Column::Id)
                .update_columns(parceiro_frota::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
