use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::tanque;

pub async fn upsert_tanques(
    db: &DatabaseConnection,
    tanques: &[hill_common::entity::Tanque],
) -> Result<(), DbErr> {
    if tanques.is_empty() {
        return Ok(());
    }

    let active_models: Vec<tanque::ActiveModel> = tanques
        .iter()
        .map(|t| tanque::ActiveModel {
            id: ActiveValue::Set(t.id),
            numero: ActiveValue::Set(t.numero),
            gtin: ActiveValue::Set(t.gtin.clone()),
            descricao: ActiveValue::Set(t.descricao.clone()),
            capacidade: ActiveValue::Set(t.capacidade),
            estoque: ActiveValue::Set(t.estoque),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    tanque::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(tanque::Column::Id)
                .update_columns(tanque::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
