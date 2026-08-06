use hill_common::entity::parceiro_dependente;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_parceiro_dependentes(
    db: &DatabaseConnection,
    dependentes: &[hill_common::entity::ParceiroDependente],
) -> Result<(), DbErr> {
    if dependentes.is_empty() {
        return Ok(());
    }

    for d in dependentes {
        if let Some(existing) = parceiro_dependente::Entity::find_by_id(d.id)
            .one(db)
            .await?
        {
            let existing: parceiro_dependente::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        parceiro_dependente::ActiveModel {
            id: ActiveValue::Set(d.id),
            status: ActiveValue::Set(d.status.clone()),
            parceiro_id: ActiveValue::Set(d.parceiro_id),
            nome: ActiveValue::Set(d.nome.clone()),
            rfid: ActiveValue::Set(d.rfid.clone()),
            limite_disponivel: ActiveValue::Set(d.limite_disponivel),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
