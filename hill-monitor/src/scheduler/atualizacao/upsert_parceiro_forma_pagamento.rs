use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::parceiro_forma_pagamento;

pub async fn upsert_parceiro_formas_pagamento(
    db: &DatabaseConnection,
    formas: &[hill_common::entity::ParceiroFormaPagamento],
) -> Result<(), DbErr> {
    if formas.is_empty() {
        return Ok(());
    }

    let active_models: Vec<parceiro_forma_pagamento::ActiveModel> = formas
        .iter()
        .map(|f| parceiro_forma_pagamento::ActiveModel {
            id: ActiveValue::Set(f.id),
            parceiro_id: ActiveValue::Set(f.parceiro_id),
            forma_pagamento_id: ActiveValue::Set(f.forma_pagamento_id),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    parceiro_forma_pagamento::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(parceiro_forma_pagamento::Column::Id)
                .update_columns(parceiro_forma_pagamento::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
