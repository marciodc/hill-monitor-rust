use crate::web::service::response::ApiResponse;
use hill_common::entity::{
    produto, produto_setor, tabela_preco, tabela_preco_item, usuario, vendedor, Produto,
    TabelaPreco, Usuario, Vendedor,
};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, QueryFilter, QueryOrder};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ProdutoPesquisaItem {
    pub id: i32,
    pub codigo: String,
    pub descricao: String,
    pub unidade: Option<String>,
    pub valor: Decimal,
}

pub struct PesquisaService {
    db: DatabaseConnection,
}

impl PesquisaService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn pesquisa_usuario(
        &self,
        nome: Option<String>,
        ids: Option<String>,
    ) -> ApiResponse<Vec<Usuario>> {
        let mut query = usuario::Entity::find().filter(usuario::Column::Status.ne("I"));

        if let Some(ids) = ids {
            let values: Vec<i32> = ids
                .split(',')
                .filter_map(|value| value.trim().parse::<i32>().ok())
                .collect();
            if !values.is_empty() {
                query = query.filter(usuario::Column::Id.is_in(values));
            }
        }

        if let Some(nome) = nome.filter(|n| !n.trim().is_empty()) {
            query = query.filter(usuario::Column::Nome.contains(&nome));
        }

        let usuarios = query
            .order_by_asc(usuario::Column::Nome)
            .all(&self.db)
            .await
            .unwrap_or_default();

        ApiResponse::ok(usuarios)
    }

    pub async fn pesquisa_vendedor(
        &self,
        nome: Option<String>,
        codigo: Option<i32>,
    ) -> ApiResponse<Vec<Vendedor>> {
        let mut query = vendedor::Entity::find();

        if let Some(codigo) = codigo.filter(|codigo| *codigo > 0) {
            query = query.filter(vendedor::Column::Codigo.eq(codigo));
        }

        if let Some(nome) = nome.filter(|n| !n.trim().is_empty()) {
            query = query.filter(vendedor::Column::Nome.contains(&nome));
        }

        let vendedores = query
            .order_by_asc(vendedor::Column::Codigo)
            .all(&self.db)
            .await
            .unwrap_or_default();

        ApiResponse::ok(vendedores)
    }

    pub async fn pesquisa_tabela_preco(
        &self,
        descricao: Option<String>,
        id: Option<i32>,
    ) -> ApiResponse<Vec<TabelaPreco>> {
        let mut query = tabela_preco::Entity::find();

        if let Some(id) = id.filter(|id| *id != 0) {
            query = query.filter(tabela_preco::Column::Id.eq(id));
        } else if let Some(descricao) = descricao.filter(|n| !n.trim().is_empty()) {
            query = query.filter(tabela_preco::Column::Descricao.contains(&descricao));
        }

        let tabelas = query
            .order_by_asc(tabela_preco::Column::Descricao)
            .all(&self.db)
            .await
            .unwrap_or_default();

        ApiResponse::ok(tabelas)
    }

    pub async fn pesquisa_produto(
        &self,
        setor_id: i32,
        descricao: Option<String>,
        pagina: Option<i32>,
    ) -> ApiResponse<Vec<ProdutoPesquisaItem>> {
        let Some(busca) = descricao.filter(|d| !d.trim().is_empty()) else {
            return ApiResponse::ok(Vec::new());
        };

        let tabela_padrao = tabela_preco::Entity::find()
            .filter(tabela_preco::Column::Padrao.eq("T"))
            .filter(tabela_preco::Column::Status.eq("T"))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(|tp| tp.id);

        let produto_ids_setor = produto_setor::Entity::find()
            .filter(produto_setor::Column::SetorId.eq(setor_id))
            .all(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| row.produto_id)
            .collect::<Vec<_>>();

        let mut query = produto::Entity::find()
            .filter(produto::Column::Id.is_in(produto_ids_setor))
            .filter(produto::Column::ExclusivoKit.is_null().or(produto::Column::ExclusivoKit.ne("T")))
            .filter(
                produto::Column::GtinComercial
                    .contains(&busca)
                    .or(produto::Column::CodigoAuxiliar.contains(&busca))
                    .or(produto::Column::Descricao.contains(&busca)),
            )
            .order_by_asc(produto::Column::Descricao);

        if let Some(tabela_padrao) = tabela_padrao {
            let produto_ids_tabela = tabela_preco_item::Entity::find()
                .filter(tabela_preco_item::Column::TabelaPrecoId.eq(tabela_padrao))
                .all(&self.db)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|item| item.produto_id)
                .collect::<Vec<_>>();
            query = query.filter(produto::Column::Id.is_in(produto_ids_tabela));
        }

        let produtos = query
            .all(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .skip(((std::cmp::max(pagina.unwrap_or(1), 1) - 1) * 20) as usize)
            .take(20)
            .collect::<Vec<Produto>>();

        let tabela_items = if let Some(id) = tabela_padrao {
            tabela_preco_item::Entity::find()
                .filter(tabela_preco_item::Column::TabelaPrecoId.eq(id))
                .all(&self.db)
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let itens = produtos
            .into_iter()
            .map(|produto| {
                let valor = tabela_items
                    .iter()
                    .find(|item| item.produto_id == produto.id)
                    .map(|item| item.valor_comercial)
                    .unwrap_or(Decimal::ZERO);

                ProdutoPesquisaItem {
                    id: produto.id,
                    codigo: produto
                        .gtin_comercial
                        .clone()
                        .or(produto.codigo_auxiliar.clone())
                        .unwrap_or_default(),
                    descricao: produto.descricao,
                    unidade: produto.unidade_comercial,
                    valor,
                }
            })
            .collect();

        ApiResponse::ok(itens)
    }
}
