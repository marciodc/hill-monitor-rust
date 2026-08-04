use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::parceiro_dependente;

pub async fn upsert_parceiro_dependentes(
    db: &DatabaseConnection,
    dependentes: &[hill_common::entity::ParceiroDependente],
) -> Result<(), DbErr> {
    if dependentes.is_empty() {
        return Ok(());
    }

    let active_models: Vec<parceiro_dependente::ActiveModel> = dependentes
        .iter()
        .map(|d| parceiro_dependente::ActiveModel {
            id: ActiveValue::Set(d.id),
            status: ActiveValue::Set(d.status.clone()),
            parceiro_id: ActiveValue::Set(d.parceiro_id),
            nome: ActiveValue::Set(d.nome.clone()),
            rfid: ActiveValue::Set(d.rfid.clone()),
            limite_disponivel: ActiveValue::Set(d.limite_disponivel),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    parceiro_dependente::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(parceiro_dependente::Column::Id)
                .update_columns(parceiro_dependente::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
