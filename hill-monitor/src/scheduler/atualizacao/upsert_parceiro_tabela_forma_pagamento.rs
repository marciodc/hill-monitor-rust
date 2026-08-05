use hill_common::entity::parceiro_tabela_forma_pagamento;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_parceiro_tabelas_formas_pagamento(
    db: &DatabaseConnection,
    tabelas: &[hill_common::entity::ParceiroTabelaFormaPagamento],
) -> Result<(), DbErr> {
    if tabelas.is_empty() {
        return Ok(());
    }

    for t in tabelas {
        if let Some(existing) = parceiro_tabela_forma_pagamento::Entity::find_by_id(t.id).one(db).await? {
            let existing: parceiro_tabela_forma_pagamento::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        parceiro_tabela_forma_pagamento::ActiveModel {
            id: ActiveValue::Set(t.id),
            status: ActiveValue::Set(t.status.clone()),
            parceiro_id: ActiveValue::Set(t.parceiro_id),
            forma_pagamento_id: ActiveValue::Set(t.forma_pagamento_id),
            tabela_id: ActiveValue::Set(t.tabela_id),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
