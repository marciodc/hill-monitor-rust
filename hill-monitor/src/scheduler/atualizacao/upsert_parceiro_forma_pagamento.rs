use hill_common::entity::parceiro_forma_pagamento;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_parceiro_formas_pagamento(
    db: &DatabaseConnection,
    formas: &[hill_common::entity::ParceiroFormaPagamento],
) -> Result<(), DbErr> {
    if formas.is_empty() {
        return Ok(());
    }

    for f in formas {
        if let Some(existing) = parceiro_forma_pagamento::Entity::find_by_id(f.id)
            .one(db)
            .await?
        {
            let existing: parceiro_forma_pagamento::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        parceiro_forma_pagamento::ActiveModel {
            id: ActiveValue::Set(f.id),
            parceiro_id: ActiveValue::Set(f.parceiro_id),
            forma_pagamento_id: ActiveValue::Set(f.forma_pagamento_id),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
