use crate::entity::{Configuracao, configuracao, parametro};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct ConfigHelper {
    db: DatabaseConnection,
}

impl ConfigHelper {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_configuracao(&self) -> Result<Configuracao, DbErr> {
        configuracao::Entity::find()
            .one(&self.db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("Configuração não encontrada".to_string()))
    }

    pub async fn get_config_by_pdv(&self, pdv: Uuid) -> Result<Option<Configuracao>, DbErr> {
        configuracao::Entity::find_by_id(pdv).one(&self.db).await
    }

    pub async fn list_configuracoes(&self) -> Result<Vec<Configuracao>, DbErr> {
        configuracao::Entity::find().all(&self.db).await
    }

    pub async fn get_parametro(
        &self,
        chave: &str,
        pdv: Option<Uuid>,
    ) -> Result<Option<String>, DbErr> {
        let mut query = parametro::Entity::find().filter(parametro::Column::Chave.eq(chave));
        if let Some(pdv_id) = pdv {
            query = query.filter(parametro::Column::Pdv.eq(pdv_id));
        } else {
            query = query.filter(parametro::Column::Pdv.is_null());
        }
        let param = query.one(&self.db).await?;
        Ok(param.map(|p| p.valor))
    }

    pub async fn set_parametro(
        &self,
        chave: &str,
        valor: &str,
        pdv: Option<Uuid>,
    ) -> Result<(), DbErr> {
        // Delete existing
        let mut delete_query =
            parametro::Entity::delete_many().filter(parametro::Column::Chave.eq(chave));
        if let Some(pdv_id) = pdv {
            delete_query = delete_query.filter(parametro::Column::Pdv.eq(pdv_id));
        } else {
            delete_query = delete_query.filter(parametro::Column::Pdv.is_null());
        }
        delete_query.exec(&self.db).await?;

        // Insert new
        let new_param = parametro::ActiveModel {
            id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
            pdv: sea_orm::ActiveValue::Set(pdv),
            chave: sea_orm::ActiveValue::Set(chave.to_string()),
            valor: sea_orm::ActiveValue::Set(valor.to_string()),
        };
        parametro::Entity::insert(new_param).exec(&self.db).await?;
        Ok(())
    }
}
