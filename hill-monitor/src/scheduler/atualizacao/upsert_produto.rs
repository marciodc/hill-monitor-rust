use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveValue, sea_query::OnConflict};
use hill_common::entity::produto;

pub async fn upsert_produtos(db: &DatabaseConnection, produtos: &[hill_common::entity::Produto]) -> Result<(), DbErr> {
    if produtos.is_empty() {
        return Ok(());
    }

    let active_models: Vec<produto::ActiveModel> = produtos.iter().map(|prod| {
        produto::ActiveModel {
            id: ActiveValue::Set(prod.id),
            tipo: ActiveValue::Set(prod.tipo.clone()),
            categoria: ActiveValue::Set(prod.categoria.clone()),
            unidade_tributacao: ActiveValue::Set(prod.unidade_tributacao.clone()),
            descricao: ActiveValue::Set(prod.descricao.clone()),
            descricao_resumida: ActiveValue::Set(prod.descricao_resumida.clone()),
            gtin_tributacao: ActiveValue::Set(prod.gtin_tributacao.clone()),
            gtin_comercial: ActiveValue::Set(prod.gtin_comercial.clone()),
            unidade_comercial: ActiveValue::Set(prod.unidade_comercial.clone()),
            quantidade_tributacao: ActiveValue::Set(prod.quantidade_tributacao),
            ncm: ActiveValue::Set(prod.ncm.clone()),
            ncm_excecao: ActiveValue::Set(prod.ncm_excecao.clone()),
            imposto_aliquota_importacao: ActiveValue::Set(prod.imposto_aliquota_importacao),
            imposto_aliquota_federal: ActiveValue::Set(prod.imposto_aliquota_federal),
            imposto_aliquota_estadual: ActiveValue::Set(prod.imposto_aliquota_estadual),
            imposto_aliquota_municipal: ActiveValue::Set(prod.imposto_aliquota_municipal),
            imposto_chave: ActiveValue::Set(prod.imposto_chave.clone()),
            tipo_codigo: ActiveValue::Set(prod.tipo_codigo),
            codigo: ActiveValue::Set(prod.codigo.clone()),
            codigo_auxiliar: ActiveValue::Set(prod.codigo_auxiliar.clone()),
            indicador_producao: ActiveValue::Set(prod.indicador_producao.clone()),
            fracionado: ActiveValue::Set(prod.fracionado.clone()),
            pesado_caixa: ActiveValue::Set(prod.pesado_caixa.clone()),
            cst: ActiveValue::Set(prod.cst.clone()),
            cst_pis: ActiveValue::Set(prod.cst_pis.clone()),
            cst_cofins: ActiveValue::Set(prod.cst_cofins.clone()),
            observacao: ActiveValue::Set(prod.observacao.clone()),
            codigo_anp: ActiveValue::Set(prod.codigo_anp),
            descricao_anp: ActiveValue::Set(prod.descricao_anp.clone()),
            solicita_vendedor: ActiveValue::Set(prod.solicita_vendedor.clone()),
            grade_id: ActiveValue::Set(prod.grade_id),
            controla_numero_serie: ActiveValue::Set(prod.controla_numero_serie.clone()),
            controla_lote: ActiveValue::Set(prod.controla_lote.clone()),
            setor_impressao_1: ActiveValue::Set(prod.setor_impressao_1),
            setor_impressao_2: ActiveValue::Set(prod.setor_impressao_2),
            setor_impressao_3: ActiveValue::Set(prod.setor_impressao_3),
            setor_impressao_4: ActiveValue::Set(prod.setor_impressao_4),
            exclusivo_kit: ActiveValue::Set(prod.exclusivo_kit.clone()),
            cest: ActiveValue::Set(prod.cest.clone()),
            cfop: ActiveValue::Set(prod.cfop),
            aliquota: ActiveValue::Set(prod.aliquota),
            aliquota_cofins: ActiveValue::Set(prod.aliquota_cofins),
            aliquota_pis: ActiveValue::Set(prod.aliquota_pis),
            tipo_combustivel: ActiveValue::Set(prod.tipo_combustivel),
            etiqueta_balanca: ActiveValue::Set(prod.etiqueta_balanca.clone()),
            predbcefet: ActiveValue::Set(prod.predbcefet),
            picmsefet: ActiveValue::Set(prod.picmsefet),
            pfcpstret: ActiveValue::Set(prod.pfcpstret),
            pfcpst: ActiveValue::Set(prod.pfcpst),
            pfcp: ActiveValue::Set(prod.pfcp),
            modbc: ActiveValue::Set(prod.modbc),
            modbcst: ActiveValue::Set(prod.modbcst),
            pmvast: ActiveValue::Set(prod.pmvast),
            predbcst: ActiveValue::Set(prod.predbcst),
            picmsst: ActiveValue::Set(prod.picmsst),
            predbc: ActiveValue::Set(prod.predbc),
            pglp: ActiveValue::Set(prod.pglp),
            pgnn: ActiveValue::Set(prod.pgnn),
            pgni: ActiveValue::Set(prod.pgni),
            vpart: ActiveValue::Set(prod.vpart),
        }
    }).collect();

    use sea_orm::{Iterable, IdenStatic};
    produto::Entity::insert_many(active_models)
        .on_conflict(
            OnConflict::column(produto::Column::Id)
                .update_columns(produto::Column::iter().filter(|col| col.as_str() != "id"))
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}
