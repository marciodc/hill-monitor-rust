use hill_common::entity::tanque;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_tanques(
    db: &DatabaseConnection,
    tanques: &[hill_common::entity::Tanque],
) -> Result<(), DbErr> {
    if tanques.is_empty() {
        return Ok(());
    }

    for t in tanques {
        if let Some(existing) = tanque::Entity::find_by_id(t.id).one(db).await? {
            let existing: tanque::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        tanque::ActiveModel {
            id: ActiveValue::Set(t.id),
            numero: ActiveValue::Set(t.numero),
            gtin: ActiveValue::Set(t.gtin.clone()),
            descricao: ActiveValue::Set(t.descricao.clone()),
            capacidade: ActiveValue::Set(t.capacidade),
            estoque: ActiveValue::Set(t.estoque),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
