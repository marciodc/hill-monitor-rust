use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::tabela_preco_item;

pub async fn upsert_tabela_preco_itens(
    db: &DatabaseConnection,
    itens: &[hill_common::entity::TabelaPrecoItem],
) -> Result<(), DbErr> {
    if itens.is_empty() {
        return Ok(());
    }

    let active_models: Vec<tabela_preco_item::ActiveModel> = itens
        .iter()
        .map(|i| tabela_preco_item::ActiveModel {
            id: ActiveValue::Set(i.id),
            tabela_preco_id: ActiveValue::Set(i.tabela_preco_id),
            produto_id: ActiveValue::Set(i.produto_id),
            valor_comercial: ActiveValue::Set(i.valor_comercial),
            valor_tributacao: ActiveValue::Set(i.valor_tributacao),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    tabela_preco_item::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(tabela_preco_item::Column::Id)
                .update_columns(tabela_preco_item::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
