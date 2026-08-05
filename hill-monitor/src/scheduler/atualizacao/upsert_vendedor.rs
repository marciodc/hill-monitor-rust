use hill_common::entity::vendedor;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_vendedores(
    db: &DatabaseConnection,
    vendedores: &[hill_common::entity::Vendedor],
) -> Result<(), DbErr> {
    if vendedores.is_empty() {
        return Ok(());
    }

    for v in vendedores {
        if let Some(existing) = vendedor::Entity::find_by_id(v.id).one(db).await? {
            let existing: vendedor::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        vendedor::ActiveModel {
            id: ActiveValue::Set(v.id),
            codigo: ActiveValue::Set(v.codigo),
            nome: ActiveValue::Set(v.nome.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
