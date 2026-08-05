use hill_common::entity::tabela_preco;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_tabela_precos(
    db: &DatabaseConnection,
    precos: &[hill_common::entity::TabelaPreco],
) -> Result<(), DbErr> {
    if precos.is_empty() {
        return Ok(());
    }

    for t in precos {
        if let Some(existing) = tabela_preco::Entity::find_by_id(t.id).one(db).await? {
            let existing: tabela_preco::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        tabela_preco::ActiveModel {
            id: ActiveValue::Set(t.id),
            status: ActiveValue::Set(t.status.clone()),
            padrao: ActiveValue::Set(t.padrao.clone()),
            descricao: ActiveValue::Set(t.descricao.clone()),
            exclusiva_cliente: ActiveValue::Set(t.exclusiva_cliente.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
