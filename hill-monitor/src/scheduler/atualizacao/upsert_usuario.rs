use hill_common::entity::usuario;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_usuarios(
    db: &DatabaseConnection,
    usuarios: &[hill_common::entity::Usuario],
) -> Result<(), DbErr> {
    if usuarios.is_empty() {
        return Ok(());
    }

    for user in usuarios {
        if let Some(existing) = usuario::Entity::find_by_id(user.id).one(db).await? {
            let existing: usuario::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

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
        .insert(db)
        .await?;
    }

    Ok(())
}
