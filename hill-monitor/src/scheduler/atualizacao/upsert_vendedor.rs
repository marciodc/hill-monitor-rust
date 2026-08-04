use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::vendedor;

pub async fn upsert_vendedores(
    db: &DatabaseConnection,
    vendedores: &[hill_common::entity::Vendedor],
) -> Result<(), DbErr> {
    if vendedores.is_empty() {
        return Ok(());
    }

    let active_models: Vec<vendedor::ActiveModel> = vendedores
        .iter()
        .map(|v| vendedor::ActiveModel {
            id: ActiveValue::Set(v.id),
            codigo: ActiveValue::Set(v.codigo),
            nome: ActiveValue::Set(v.nome.clone()),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    vendedor::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(vendedor::Column::Id)
                .update_columns(vendedor::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
