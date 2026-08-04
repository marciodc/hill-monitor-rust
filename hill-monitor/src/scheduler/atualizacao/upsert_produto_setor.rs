use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::produto_setor;

pub async fn upsert_produtos_setores(
    db: &DatabaseConnection,
    setores: &[hill_common::entity::ProdutoSetor],
) -> Result<(), DbErr> {
    if setores.is_empty() {
        return Ok(());
    }

    let active_models: Vec<produto_setor::ActiveModel> = setores
        .iter()
        .map(|s| produto_setor::ActiveModel {
            id: ActiveValue::Set(s.id),
            setor_id: ActiveValue::Set(s.setor_id),
            produto_id: ActiveValue::Set(s.produto_id),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    produto_setor::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(produto_setor::Column::Id)
                .update_columns(produto_setor::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
