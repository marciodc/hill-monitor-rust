use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::parceiro_tabela_forma_pagamento;

pub async fn upsert_parceiro_tabelas_formas_pagamento(
    db: &DatabaseConnection,
    tabelas: &[hill_common::entity::ParceiroTabelaFormaPagamento],
) -> Result<(), DbErr> {
    if tabelas.is_empty() {
        return Ok(());
    }

    let active_models: Vec<parceiro_tabela_forma_pagamento::ActiveModel> = tabelas
        .iter()
        .map(|t| parceiro_tabela_forma_pagamento::ActiveModel {
            id: ActiveValue::Set(t.id),
            status: ActiveValue::Set(t.status.clone()),
            parceiro_id: ActiveValue::Set(t.parceiro_id),
            forma_pagamento_id: ActiveValue::Set(t.forma_pagamento_id),
            tabela_id: ActiveValue::Set(t.tabela_id),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    parceiro_tabela_forma_pagamento::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(parceiro_tabela_forma_pagamento::Column::Id)
                .update_columns(parceiro_tabela_forma_pagamento::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
