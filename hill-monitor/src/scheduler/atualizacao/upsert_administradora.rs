use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::administradora;

pub async fn upsert_administradoras(
    db: &DatabaseConnection,
    administradoras: &[hill_common::entity::Administradora],
) -> Result<(), DbErr> {
    if administradoras.is_empty() {
        return Ok(());
    }

    let active_models: Vec<administradora::ActiveModel> = administradoras
        .iter()
        .map(|a| administradora::ActiveModel {
            id: ActiveValue::Set(a.id),
            bandeira: ActiveValue::Set(a.bandeira.clone()),
            descricao: ActiveValue::Set(a.descricao.clone()),
            cnpj: ActiveValue::Set(a.cnpj.clone()),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    administradora::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(administradora::Column::Id)
                .update_columns(administradora::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
