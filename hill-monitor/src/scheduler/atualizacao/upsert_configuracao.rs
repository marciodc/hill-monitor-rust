use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use hill_common::entity::configuracao;

pub async fn upsert_configuracoes(db: &DatabaseConnection, configs: &[hill_common::entity::Configuracao]) -> Result<(), DbErr> {
    if configs.is_empty() {
        return Ok(());
    }

    for config in configs {
        if let Some(existing) = configuracao::Entity::find_by_id(config.id).one(db).await? {
            let existing: configuracao::ActiveModel = existing.into();
            existing.delete(db).await?;
        }

        configuracao::ActiveModel {
            id: ActiveValue::Set(config.id),
            pdv_numero: ActiveValue::Set(config.pdv_numero),
            empresa: ActiveValue::Set(config.empresa),
            setor: ActiveValue::Set(config.setor),
            razao_social: ActiveValue::Set(config.razao_social.clone()),
            nome_fantasia: ActiveValue::Set(config.nome_fantasia.clone()),
            cnpj: ActiveValue::Set(config.cnpj.clone()),
            inscricao_estadual: ActiveValue::Set(config.inscricao_estadual.clone()),
            inscricao_municipal: ActiveValue::Set(config.inscricao_municipal.clone()),
            cnae: ActiveValue::Set(config.cnae.clone()),
            codigo_regime_tributacao: ActiveValue::Set(config.codigo_regime_tributacao),
            logradouro: ActiveValue::Set(config.logradouro.clone()),
            complemento: ActiveValue::Set(config.complemento.clone()),
            numero: ActiveValue::Set(config.numero.clone()),
            bairro: ActiveValue::Set(config.bairro.clone()),
            municipio: ActiveValue::Set(config.municipio.clone()),
            cod_municipio: ActiveValue::Set(config.cod_municipio),
            uf: ActiveValue::Set(config.uf.clone()),
            cep: ActiveValue::Set(config.cep.clone()),
            fone: ActiveValue::Set(config.fone.clone()),
            mensagem_venda: ActiveValue::Set(config.mensagem_venda.clone()),
            exibir_valor_fechamento_caixa: ActiveValue::Set(config.exibir_valor_fechamento_caixa.clone()),
            exibir_valor_sangria: ActiveValue::Set(config.exibir_valor_sangria.clone()),
            solicita_senha_venda: ActiveValue::Set(config.solicita_senha_venda.clone()),
            identifica_vendedor: ActiveValue::Set(config.identifica_vendedor.clone()),
            diferenca_abastecimento: ActiveValue::Set(config.diferenca_abastecimento),
            quantidade_maxima_gerada: ActiveValue::Set(config.quantidade_maxima_gerada),
            quantidade_maxima_abastecimento: ActiveValue::Set(config.quantidade_maxima_abastecimento),
            tipo_estabelecimento: ActiveValue::Set(config.tipo_estabelecimento.clone()),
            tipo_busca_abastecimento: ActiveValue::Set(config.tipo_busca_abastecimento),
            tipo_identificacao_cliente: ActiveValue::Set(config.tipo_identificacao_cliente),
            tipo_identificacao_fidelidade: ActiveValue::Set(config.tipo_identificacao_fidelidade),
            tipo_identificacao_usuario: ActiveValue::Set(config.tipo_identificacao_usuario),
            desconto_fechamento: ActiveValue::Set(config.desconto_fechamento.clone()),
            imprime_gerencial_fidelidade: ActiveValue::Set(config.imprime_gerencial_fidelidade.clone()),
            imprime_gerencial_promocao: ActiveValue::Set(config.imprime_gerencial_promocao.clone()),
            imprime_espelho_completo: ActiveValue::Set(config.imprime_espelho_completo.clone()),
            imprime_espelho_vencimento: ActiveValue::Set(config.imprime_espelho_vencimento.clone()),
            imprime_recibo_espelho: ActiveValue::Set(config.imprime_recibo_espelho.clone()),
            imprime_rel_fechamento_caixa: ActiveValue::Set(config.imprime_rel_fechamento_caixa.clone()),
            imprime_rel_fechamento_turno: ActiveValue::Set(config.imprime_rel_fechamento_turno.clone()),
            imprime_descricao_grade: ActiveValue::Set(config.imprime_descricao_grade.clone()),
            imprime_espelho_sangria: ActiveValue::Set(config.imprime_espelho_sangria.clone()),
            imprime_espelho_suprimento: ActiveValue::Set(config.imprime_espelho_suprimento.clone()),
            codigo_balanca: ActiveValue::Set(config.codigo_balanca.clone()),
            abre_venda_consulta_produto: ActiveValue::Set(config.abre_venda_consulta_produto.clone()),
            vias_espelho: ActiveValue::Set(config.vias_espelho),
            pedido_agrupado: ActiveValue::Set(config.pedido_agrupado.clone()),
            pre_venda_pagamento: ActiveValue::Set(config.pre_venda_pagamento.clone()),
            alterar_pre_venda: ActiveValue::Set(config.alterar_pre_venda.clone()),
            atualizacao: ActiveValue::Set(config.atualizacao),
            versao_retaguarda: ActiveValue::Set(config.versao_retaguarda.clone()),
            senha_usuario_ativo: ActiveValue::Set(config.senha_usuario_ativo.clone()),
            efetuar_sangria_usuario: ActiveValue::Set(config.efetuar_sangria_usuario.clone()),
            vlr_max_nfce: ActiveValue::Set(config.vlr_max_nfce),
            exibir_limite_cliente: ActiveValue::Set(config.exibir_limite_cliente.clone()),
            emissao_direta_nf_pj: ActiveValue::Set(config.emissao_direta_nf_pj.clone()),
            lista_todos_abastecimentos_pdv: ActiveValue::Set(config.lista_todos_abastecimentos_pdv.clone()),
            id_token: ActiveValue::Set(config.id_token.clone()),
            token_csc: ActiveValue::Set(config.token_csc.clone()),
            controle_estoque_combustivel: ActiveValue::Set(config.controle_estoque_combustivel.clone()),
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
