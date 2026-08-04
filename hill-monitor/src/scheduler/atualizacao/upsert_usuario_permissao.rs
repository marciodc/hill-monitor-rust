use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::usuario_permissao;

pub async fn upsert_usuario_permissoes(
    db: &DatabaseConnection,
    permissoes: &[hill_common::entity::UsuarioPermissao],
) -> Result<(), DbErr> {
    if permissoes.is_empty() {
        return Ok(());
    }

    let active_models: Vec<usuario_permissao::ActiveModel> = permissoes
        .iter()
        .map(|p| usuario_permissao::ActiveModel {
            id: ActiveValue::Set(p.id),
            usuario_id: ActiveValue::Set(p.usuario_id),
            cancela_venda_aberta: ActiveValue::Set(p.cancela_venda_aberta.clone()),
            cancela_venda_fechada: ActiveValue::Set(p.cancela_venda_fechada.clone()),
            cancela_item: ActiveValue::Set(p.cancela_item.clone()),
            desconto_item: ActiveValue::Set(p.desconto_item.clone()),
            desconto_fechamento: ActiveValue::Set(p.desconto_fechamento.clone()),
            acrescimo_fechamento: ActiveValue::Set(p.acrescimo_fechamento.clone()),
            acrescimo_item: ActiveValue::Set(p.acrescimo_item.clone()),
            cliente_limite: ActiveValue::Set(p.cliente_limite.clone()),
            cliente_bloqueado: ActiveValue::Set(p.cliente_bloqueado.clone()),
            cliente_forma_pagamento: ActiveValue::Set(p.cliente_forma_pagamento.clone()),
            sangria: ActiveValue::Set(p.sangria.clone()),
            suprimento: ActiveValue::Set(p.suprimento.clone()),
            abertura_turno: ActiveValue::Set(p.abertura_turno.clone()),
            fechamento_turno: ActiveValue::Set(p.fechamento_turno.clone()),
            afericao: ActiveValue::Set(p.afericao.clone()),
            lista_todos_abastecimentos: ActiveValue::Set(p.lista_todos_abastecimentos.clone()),
            operacoes_tef: ActiveValue::Set(p.operacoes_tef.clone()),
            desmembramento: ActiveValue::Set(p.desmembramento.clone()),
            libera_troco_maximo: ActiveValue::Set(p.libera_troco_maximo.clone()),
        })
        .collect();

    use sea_orm::{Iterable, IdenStatic};
    usuario_permissao::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(usuario_permissao::Column::Id)
                .update_columns(usuario_permissao::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}
