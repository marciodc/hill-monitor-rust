use hill_common::entity::produto_setor;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_produtos_setores(
    db: &DatabaseConnection,
    setores: &[hill_common::entity::ProdutoSetor],
) -> Result<(), DbErr> {
    if setores.is_empty() {
        return Ok(());
    }

    for s in setores {
        if let Some(existing) = produto_setor::Entity::find_by_id(s.id).one(db).await? {
            let existing: produto_setor::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        produto_setor::ActiveModel {
            id: ActiveValue::Set(s.id),
            setor_id: ActiveValue::Set(s.setor_id),
            produto_id: ActiveValue::Set(s.produto_id),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
