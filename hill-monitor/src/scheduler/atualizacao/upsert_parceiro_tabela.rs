use hill_common::entity::parceiro_tabela;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_parceiro_tabelas(
    db: &DatabaseConnection,
    tabelas: &[hill_common::entity::ParceiroTabela],
) -> Result<(), DbErr> {
    if tabelas.is_empty() {
        return Ok(());
    }

    for t in tabelas {
        if let Some(existing) = parceiro_tabela::Entity::find_by_id(t.id).one(db).await? {
            let existing: parceiro_tabela::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        parceiro_tabela::ActiveModel {
            id: ActiveValue::Set(t.id),
            status: ActiveValue::Set(t.status.clone()),
            parceiro_id: ActiveValue::Set(t.parceiro_id),
            tabela_id: ActiveValue::Set(t.tabela_id),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
