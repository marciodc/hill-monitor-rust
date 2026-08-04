use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::tabela_preco;

pub async fn upsert_tabela_precos(
    db: &DatabaseConnection,
    precos: &[hill_common::entity::TabelaPreco],
) -> Result<(), DbErr> {
    if precos.is_empty() {
        return Ok(());
    }

    let active_models: Vec<tabela_preco::ActiveModel> = precos
        .iter()
        .map(|t| tabela_preco::ActiveModel {
            id: ActiveValue::Set(t.id),
            status: ActiveValue::Set(t.status.clone()),
            padrao: ActiveValue::Set(t.padrao.clone()),
            descricao: ActiveValue::Set(t.descricao.clone()),
            exclusiva_cliente: ActiveValue::Set(t.exclusiva_cliente.clone()),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    tabela_preco::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(tabela_preco::Column::Id)
                .update_columns(tabela_preco::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
