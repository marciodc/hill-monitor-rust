use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::setor;

pub async fn upsert_setores(
    db: &DatabaseConnection,
    setores: &[hill_common::entity::Setor],
) -> Result<(), DbErr> {
    if setores.is_empty() {
        return Ok(());
    }

    let active_models: Vec<setor::ActiveModel> = setores
        .iter()
        .map(|s| setor::ActiveModel {
            id: ActiveValue::Set(s.id),
            descricao: ActiveValue::Set(s.descricao.clone()),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    setor::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(setor::Column::Id)
                .update_columns(setor::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
