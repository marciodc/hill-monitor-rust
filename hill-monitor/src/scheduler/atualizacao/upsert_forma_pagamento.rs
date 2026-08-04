use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::forma_pagamento;

pub async fn upsert_formas_pagamento(db: &DatabaseConnection, formas: &[hill_common::entity::FormaPagamento]) -> Result<(), DbErr> {
    if formas.is_empty() {
        return Ok(());
    }

    let active_models: Vec<forma_pagamento::ActiveModel> = formas.iter().map(|fp| {
        forma_pagamento::ActiveModel {
            id: ActiveValue::Set(fp.id),
            numero: ActiveValue::Set(fp.numero),
            tipo_pagamento: ActiveValue::Set(fp.tipo_pagamento),
            descricao: ActiveValue::Set(fp.descricao.clone()),
            valor_aviso_sangria: ActiveValue::Set(fp.valor_aviso_sangria),
            somente_cadastrados: ActiveValue::Set(fp.somente_cadastrados.clone()),
            permite_troco: ActiveValue::Set(fp.permite_troco.clone()),
            permite_desconto: ActiveValue::Set(fp.permite_desconto.clone()),
            permite_acrescimo: ActiveValue::Set(fp.permite_acrescimo.clone()),
            dados_cheque: ActiveValue::Set(fp.dados_cheque.clone()),
            dados_tef: ActiveValue::Set(fp.dados_tef.clone()),
            maximo_parcelas: ActiveValue::Set(fp.maximo_parcelas),
            tef_rede: ActiveValue::Set(fp.tef_rede.clone()),
            tef_operacao: ActiveValue::Set(fp.tef_operacao),
            voucher: ActiveValue::Set(fp.voucher.clone()),
            ignora_limite_troco: ActiveValue::Set(fp.ignora_limite_troco.clone()),
            solicita_vencimento: ActiveValue::Set(fp.solicita_vencimento.clone()),
            valida_limite_credito: ActiveValue::Set(fp.valida_limite_credito.clone()),
            espelho: ActiveValue::Set(fp.espelho.clone()),
            dias_vencimento: ActiveValue::Set(fp.dias_vencimento.clone()),
            tipo_venda: ActiveValue::Set(fp.tipo_venda.clone()),
            tabela_id: ActiveValue::Set(fp.tabela_id),
            permite_cheque_troco: ActiveValue::Set(fp.permite_cheque_troco.clone()),
            permite_deposito_troco: ActiveValue::Set(fp.permite_deposito_troco.clone()),
            percentual_maximo_troco: ActiveValue::Set(fp.percentual_maximo_troco),
            percentual_desconto: ActiveValue::Set(fp.percentual_desconto),
            percentual_maximo_desconto: ActiveValue::Set(fp.percentual_maximo_desconto),
            venda_mobile: ActiveValue::Set(fp.venda_mobile.clone()),
            troco_em_deposito: ActiveValue::Set(fp.troco_em_deposito.clone()),
            vendas_com_juros_mobile: ActiveValue::Set(fp.vendas_com_juros_mobile.clone()),
        }
    }).collect();

    use sea_orm::{Iterable, IdenStatic};
    forma_pagamento::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(forma_pagamento::Column::Id)
                .update_columns(forma_pagamento::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}
