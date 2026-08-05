use hill_common::entity::usuario_permissao;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};

pub async fn upsert_usuario_permissoes(
    db: &DatabaseConnection,
    permissoes: &[hill_common::entity::UsuarioPermissao],
) -> Result<(), DbErr> {
    if permissoes.is_empty() {
        return Ok(());
    }

    for p in permissoes {
        if let Some(existing) = usuario_permissao::Entity::find_by_id(p.id).one(db).await? {
            let existing: usuario_permissao::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        usuario_permissao::ActiveModel {
            id: ActiveValue::Set(p.id),
            usuario_id: ActiveValue::Set(p.usuario_id),
            cancela_venda_aberta: ActiveValue::Set(p.cancela_venda_aberta.clone()),
            cancela_venda_fechada: ActiveValue::Set(p.cancela_venda_fechada.clone()),
            cancela_item: ActiveValue::Set(p.cancela_item.clone()),
            desconto_item: ActiveValue::Set(p.desconto_item.clone()),
            desconto_fechamento: ActiveValue::Set(p.desconto_fechamento.clone()),
            desconto_fechamento_pv: ActiveValue::Set(p.desconto_fechamento_pv.clone()),
            acrescimo_fechamento: ActiveValue::Set(p.acrescimo_fechamento.clone()),
            acrescimo_item: ActiveValue::Set(p.acrescimo_item.clone()),
            acrescimo_fechamento_pv: ActiveValue::Set(p.acrescimo_fechamento_pv.clone()),
            cliente_limite: ActiveValue::Set(p.cliente_limite.clone()),
            cliente_bloqueado: ActiveValue::Set(p.cliente_bloqueado.clone()),
            cliente_forma_pagamento: ActiveValue::Set(p.cliente_forma_pagamento.clone()),
            sangria: ActiveValue::Set(p.sangria.clone()),
            suprimento: ActiveValue::Set(p.suprimento.clone()),
            abertura_turno: ActiveValue::Set(p.abertura_turno.clone()),
            fechamento_turno: ActiveValue::Set(p.fechamento_turno.clone()),
            reabertura_turno: ActiveValue::Set(p.reabertura_turno.clone()),
            afericao: ActiveValue::Set(p.afericao.clone()),
            lista_todos_abastecimentos: ActiveValue::Set(p.lista_todos_abastecimentos.clone()),
            operacoes_tef: ActiveValue::Set(p.operacoes_tef.clone()),
            limite_desconto_acrescimo: ActiveValue::Set(p.limite_desconto_acrescimo.clone()),
            sangria_lancamento_saida: ActiveValue::Set(p.sangria_lancamento_saida.clone()),
            desmembramento: ActiveValue::Set(p.desmembramento.clone()),
            libera_troco_maximo: ActiveValue::Set(p.libera_troco_maximo.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
