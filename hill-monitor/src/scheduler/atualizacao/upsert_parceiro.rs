use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::parceiro;

pub async fn upsert_parceiros(db: &DatabaseConnection, parceiros: &[hill_common::entity::Parceiro]) -> Result<(), DbErr> {
    if parceiros.is_empty() {
        return Ok(());
    }

    let active_models: Vec<parceiro::ActiveModel> = parceiros.iter().map(|p| {
        parceiro::ActiveModel {
            id: ActiveValue::Set(p.id),
            status: ActiveValue::Set(p.status.clone()),
            cpf_cnpj: ActiveValue::Set(p.cpf_cnpj.clone()),
            inscricao_estadual: ActiveValue::Set(p.inscricao_estadual.clone()),
            ie_situacao: ActiveValue::Set(p.ie_situacao.clone()),
            inscricao_municipal: ActiveValue::Set(p.inscricao_municipal.clone()),
            nome_fantasia: ActiveValue::Set(p.nome_fantasia.clone()),
            razao_social: ActiveValue::Set(p.razao_social.clone()),
            logradouro: ActiveValue::Set(p.logradouro.clone()),
            complemento: ActiveValue::Set(p.complemento.clone()),
            numero: ActiveValue::Set(p.numero.clone()),
            bairro: ActiveValue::Set(p.bairro.clone()),
            municipio: ActiveValue::Set(p.municipio.clone()),
            cod_municipio: ActiveValue::Set(p.cod_municipio),
            uf: ActiveValue::Set(p.uf.clone()),
            cep: ActiveValue::Set(p.cep.clone()),
            requer_placa: ActiveValue::Set(p.requer_placa.clone()),
            requer_km: ActiveValue::Set(p.requer_km.clone()),
            requer_condutor: ActiveValue::Set(p.requer_condutor.clone()),
            desconto_venda: ActiveValue::Set(p.desconto_venda),
            limite_disponivel: ActiveValue::Set(p.limite_disponivel),
            email: ActiveValue::Set(p.email.clone()),
            rfid: ActiveValue::Set(p.rfid.clone()),
        }
    }).collect();

    use sea_orm::{Iterable, IdenStatic};
    parceiro::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(parceiro::Column::Id)
                .update_columns(parceiro::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}
