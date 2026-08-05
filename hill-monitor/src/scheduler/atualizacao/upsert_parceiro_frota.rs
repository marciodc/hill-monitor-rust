use hill_common::entity::parceiro_frota;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_parceiro_frotas(
    db: &DatabaseConnection,
    frotas: &[hill_common::entity::ParceiroFrota],
) -> Result<(), DbErr> {
    if frotas.is_empty() {
        return Ok(());
    }

    for f in frotas {
        if let Some(existing) = parceiro_frota::Entity::find_by_id(f.id).one(db).await? {
            let existing: parceiro_frota::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        parceiro_frota::ActiveModel {
            id: ActiveValue::Set(f.id),
            status: ActiveValue::Set(f.status.clone()),
            parceiro_id: ActiveValue::Set(f.parceiro_id),
            veiculo: ActiveValue::Set(f.veiculo.clone()),
            placa: ActiveValue::Set(f.placa.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
