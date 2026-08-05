use crate::web::service::response::ApiResponse;
use hill_common::entity::{abastecimento, Abastecimento};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct AbastecimentoService {
    db: DatabaseConnection,
}

impl AbastecimentoService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn listar_abastecimentos(&self) -> Result<Vec<Abastecimento>, DbErr> {
        abastecimento::Entity::find()
            .filter(abastecimento::Column::Status.eq("P"))
            .filter(abastecimento::Column::Pdv.is_null())
            .all(&self.db)
            .await
    }

    pub async fn listar_abastecimentos_usuario(
        &self,
        user_rfid: &str,
    ) -> Result<Vec<Abastecimento>, DbErr> {
        abastecimento::Entity::find()
            .filter(abastecimento::Column::Status.eq("P"))
            .filter(abastecimento::Column::RfidFrentista.eq(user_rfid))
            .filter(abastecimento::Column::Pdv.is_null())
            .all(&self.db)
            .await
    }

    pub async fn localizar_abastecimento(&self, id: &str) -> Result<Option<Abastecimento>, DbErr> {
        let Ok(abastecimento_id) = Uuid::parse_str(id) else {
            return Ok(None);
        };

        abastecimento::Entity::find_by_id(abastecimento_id)
            .filter(abastecimento::Column::Status.eq("P"))
            .one(&self.db)
            .await
    }

    pub async fn seleciona_abastecimento(
        &self,
        pdv: &str,
        abastecimento_id: &str,
    ) -> ApiResponse<()> {
        let (Ok(pdv_uuid), Ok(abast_uuid)) = (Uuid::parse_str(pdv), Uuid::parse_str(abastecimento_id))
        else {
            return ApiResponse::err("Pdv ou abastecimento inválido");
        };

        match abastecimento::Entity::find_by_id(abast_uuid)
            .filter(abastecimento::Column::Pdv.is_null())
            .one(&self.db)
            .await
        {
            Ok(Some(model)) => {
                let mut active: abastecimento::ActiveModel = model.into();
                active.pdv = ActiveValue::Set(Some(pdv_uuid));
                match active.update(&self.db).await {
                    Ok(_) => ApiResponse::ok_message("OK"),
                    Err(_) => ApiResponse::err("Erro atualizando status do abastecimento"),
                }
            }
            Ok(None) | Err(_) => ApiResponse::err("Erro atualizando status do abastecimento"),
        }
    }

    pub async fn desseleciona_abastecimento(
        &self,
        pdv: &str,
        abastecimento_id: &str,
    ) -> ApiResponse<()> {
        let (Ok(pdv_uuid), Ok(abast_uuid)) = (Uuid::parse_str(pdv), Uuid::parse_str(abastecimento_id))
        else {
            return ApiResponse::err("Pdv ou abastecimento inválido");
        };

        match abastecimento::Entity::find_by_id(abast_uuid)
            .filter(abastecimento::Column::Pdv.eq(pdv_uuid))
            .one(&self.db)
            .await
        {
            Ok(Some(model)) => {
                let mut active: abastecimento::ActiveModel = model.into();
                active.pdv = ActiveValue::Set(None);
                match active.update(&self.db).await {
                    Ok(_) => ApiResponse::ok_message("OK"),
                    Err(_) => ApiResponse::err("Erro atualizando status do abastecimento"),
                }
            }
            Ok(None) | Err(_) => ApiResponse::err("Erro atualizando status do abastecimento"),
        }
    }
}
