use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::parceiro_tabela;

pub async fn upsert_parceiro_tabelas(
    db: &DatabaseConnection,
    tabelas: &[hill_common::entity::ParceiroTabela],
) -> Result<(), DbErr> {
    if tabelas.is_empty() {
        return Ok(());
    }

    let active_models: Vec<parceiro_tabela::ActiveModel> = tabelas
        .iter()
        .map(|t| parceiro_tabela::ActiveModel {
            id: ActiveValue::Set(t.id),
            status: ActiveValue::Set(t.status.clone()),
            parceiro_id: ActiveValue::Set(t.parceiro_id),
            tabela_id: ActiveValue::Set(t.tabela_id),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    parceiro_tabela::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(parceiro_tabela::Column::Id)
                .update_columns(parceiro_tabela::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
