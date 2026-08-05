use hill_common::entity::tabela_preco_item;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_tabela_preco_itens(
    db: &DatabaseConnection,
    itens: &[hill_common::entity::TabelaPrecoItem],
) -> Result<(), DbErr> {
    if itens.is_empty() {
        return Ok(());
    }

    for i in itens {
        if let Some(existing) = tabela_preco_item::Entity::find_by_id(i.id).one(db).await? {
            let existing: tabela_preco_item::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        tabela_preco_item::ActiveModel {
            id: ActiveValue::Set(i.id),
            tabela_preco_id: ActiveValue::Set(i.tabela_preco_id),
            produto_id: ActiveValue::Set(i.produto_id),
            valor_comercial: ActiveValue::Set(i.valor_comercial),
            valor_tributacao: ActiveValue::Set(i.valor_tributacao),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
