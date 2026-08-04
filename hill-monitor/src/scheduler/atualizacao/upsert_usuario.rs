use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::usuario;

pub async fn upsert_usuarios(db: &DatabaseConnection, usuarios: &[hill_common::entity::Usuario]) -> Result<(), DbErr> {
    if usuarios.is_empty() {
        return Ok(());
    }

    let active_models: Vec<usuario::ActiveModel> = usuarios.iter().map(|user| {
        usuario::ActiveModel {
            id: ActiveValue::Set(user.id),
            status: ActiveValue::Set(user.status.clone()),
            nome: ActiveValue::Set(user.nome.clone()),
            login: ActiveValue::Set(user.login.clone()),
            senha: ActiveValue::Set(user.senha.clone()),
            rfid: ActiveValue::Set(user.rfid.clone()),
            rfid_debito: ActiveValue::Set(user.rfid_debito.clone()),
            rfid_credito: ActiveValue::Set(user.rfid_credito.clone()),
            digital: ActiveValue::Set(user.digital.clone()),
            cartao_magnetico: ActiveValue::Set(user.cartao_magnetico.clone()),
            perc_max_desc_acres_item: ActiveValue::Set(user.perc_max_desc_acres_item),
            valor_max_desc_acres_item: ActiveValue::Set(user.valor_max_desc_acres_item),
            perc_max_desc_acres_subtotal: ActiveValue::Set(user.perc_max_desc_acres_subtotal),
            valor_max_desc_acres_subtotal: ActiveValue::Set(user.valor_max_desc_acres_subtotal),
        }
    }).collect();

    use sea_orm::{Iterable, IdenStatic};
    usuario::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(usuario::Column::Id)
                .update_columns(usuario::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}
