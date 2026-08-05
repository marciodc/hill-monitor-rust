use hill_common::entity::setor;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_setores(
    db: &DatabaseConnection,
    setores: &[hill_common::entity::Setor],
) -> Result<(), DbErr> {
    if setores.is_empty() {
        return Ok(());
    }

    for s in setores {
        if let Some(existing) = setor::Entity::find_by_id(s.id).one(db).await? {
            let existing: setor::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        setor::ActiveModel {
            id: ActiveValue::Set(s.id),
            descricao: ActiveValue::Set(s.descricao.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
