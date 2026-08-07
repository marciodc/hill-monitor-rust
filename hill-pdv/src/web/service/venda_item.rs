use crate::web::service::response::ApiResponse;
use hill_common::entity::{produto, venda_item};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct VendaItemPayload {
    pub id: String,
    pub status: Option<String>,
    pub venda_id: String,
    pub sequencia: i32,
    pub pre_venda: Option<String>,
    pub produto_id: i32,
    pub produto_gtin: Option<String>,
    pub quantidade: rust_decimal::Decimal,
    pub valor_comercial: rust_decimal::Decimal,
    pub valor_tributacao: rust_decimal::Decimal,
    pub subtotal: rust_decimal::Decimal,
    pub desconto: rust_decimal::Decimal,
    pub acrescimo: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub desconto_fechamento: rust_decimal::Decimal,
    pub acrescimo_fechamento: rust_decimal::Decimal,
    pub total_fechamento: rust_decimal::Decimal,
    pub cfop: i32,
    pub cst: Option<String>,
    pub icms_aliquota: rust_decimal::Decimal,
    pub icms_valor: rust_decimal::Decimal,
    pub total_tributos: rust_decimal::Decimal,
    pub total_tributos_importacao: rust_decimal::Decimal,
    pub total_tributos_federal: rust_decimal::Decimal,
    pub total_tributos_estadual: rust_decimal::Decimal,
    pub total_tributos_municipal: rust_decimal::Decimal,
    pub abastecimento_id: Option<Uuid>,
    pub bico_id: rust_decimal::Decimal,
    pub encerrante_inicial: rust_decimal::Decimal,
    pub encerrante_final: rust_decimal::Decimal,
    pub rfid_vendedor: Option<String>,
    pub rfid_cliente: Option<String>,
    pub setor_id: rust_decimal::Decimal,
    pub grade_item_id: rust_decimal::Decimal,
    pub grade_codigo: Option<String>,
    pub grade_descricao: Option<String>,
    pub lote_id: rust_decimal::Decimal,
    pub produto_serie_id: i32,
    pub tabela_preco_id: i32,
    pub tabela_padrao: Option<String>,
    pub vendedor_id: rust_decimal::Decimal,
    pub icmsst_valor: rust_decimal::Decimal,
    pub icmsst_aliquota: rust_decimal::Decimal,
    pub cst_pis: Option<String>,
    pub pis_valor: rust_decimal::Decimal,
    pub pis_aliquota: rust_decimal::Decimal,
    pub cst_cofins: Option<String>,
    pub cofins_valor: rust_decimal::Decimal,
    pub cofins_aliquota: rust_decimal::Decimal,
    pub descricao: Option<String>,
}

pub struct VendaItemService {
    db: DatabaseConnection,
}

impl VendaItemService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn lista_itens(&self, venda_id: &str) -> ApiResponse<Vec<VendaItemPayload>> {
        let Ok(venda_uuid) = Uuid::parse_str(venda_id) else {
            return ApiResponse::err("Parâmetro não informado");
        };

        let itens = venda_item::Entity::find()
            .filter(venda_item::Column::VendaId.eq(venda_uuid))
            .filter(venda_item::Column::Status.eq("A"))
            .order_by_asc(venda_item::Column::Sequencia)
            .all(&self.db)
            .await
            .unwrap_or_default();

        let produtos = produto::Entity::find()
            .all(&self.db)
            .await
            .unwrap_or_default();

        let payload = itens
            .into_iter()
            .map(|item| {
                let descricao = produtos
                    .iter()
                    .find(|produto| produto.id == item.produto_id)
                    .map(|produto| produto.descricao.clone());

                VendaItemPayload {
                    id: item.id.to_string(),
                    status: item.status,
                    venda_id: item.venda_id.to_string(),
                    sequencia: item.sequencia,
                    pre_venda: item.pre_venda,
                    produto_id: item.produto_id,
                    produto_gtin: item.produto_gtin,
                    quantidade: item.quantidade,
                    valor_comercial: item.valor_comercial,
                    valor_tributacao: item.valor_tributacao,
                    subtotal: item.subtotal,
                    desconto: item.desconto,
                    acrescimo: item.acrescimo,
                    total: item.total,
                    desconto_fechamento: item.desconto_fechamento,
                    acrescimo_fechamento: item.acrescimo_fechamento,
                    total_fechamento: item.total_fechamento,
                    cfop: item.cfop,
                    cst: item.cst,
                    icms_aliquota: item.icms_aliquota,
                    icms_valor: item.icms_valor,
                    total_tributos: item.total_tributos,
                    total_tributos_importacao: item.total_tributos_importacao,
                    total_tributos_federal: item.total_tributos_federal,
                    total_tributos_estadual: item.total_tributos_estadual,
                    total_tributos_municipal: item.total_tributos_municipal,
                    abastecimento_id: item.abastecimento_id,
                    bico_id: item.bico_id,
                    encerrante_inicial: item.encerrante_inicial,
                    encerrante_final: item.encerrante_final,
                    rfid_vendedor: item.rfid_vendedor,
                    rfid_cliente: item.rfid_cliente,
                    setor_id: item.setor_id,
                    grade_item_id: item.grade_item_id,
                    grade_codigo: item.grade_codigo,
                    grade_descricao: item.grade_descricao,
                    lote_id: item.lote_id,
                    produto_serie_id: item.produto_serie_id,
                    tabela_preco_id: item.tabela_preco_id,
                    tabela_padrao: item.tabela_padrao,
                    vendedor_id: item.vendedor_id,
                    icmsst_valor: item.icmsst_valor,
                    icmsst_aliquota: item.icmsst_aliquota,
                    cst_pis: item.cst_pis,
                    pis_valor: item.pis_valor,
                    pis_aliquota: item.pis_aliquota,
                    cst_cofins: item.cst_cofins,
                    cofins_valor: item.cofins_valor,
                    cofins_aliquota: item.cofins_aliquota,
                    descricao,
                }
            })
            .collect();

        ApiResponse::ok(payload)
    }
}
