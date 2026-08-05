use crate::web::controller::login::{LoginResponse, LoginUser};
use hill_common::entity::{configuracao, usuario, usuario_permissao, Usuario};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use scrypt::{scrypt, Params};
use tracing::error;

pub struct LoginService {
    db: DatabaseConnection,
}

impl LoginService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn autentica(&self, login: LoginUser) -> LoginResponse {
        match self.validate_common(&login, false).await {
            Ok(usuario) => LoginResponse::ok("Usuário autenticado com sucesso.", Some(usuario)),
            Err(message) => LoginResponse::err(message),
        }
    }

    pub async fn valida_usuario(&self, login: LoginUser) -> LoginResponse {
        match self.validate_common(&login, true).await {
            Ok(usuario) => LoginResponse::ok("Usuário validado com sucesso.", Some(usuario)),
            Err(message) => LoginResponse::err(message),
        }
    }

    async fn validate_common(
        &self,
        login: &LoginUser,
        validar_acao: bool,
    ) -> Result<Usuario, String> {
        if let Some(pdv) = login.pdv {
            let existe_config = configuracao::Entity::find_by_id(pdv)
                .one(&self.db)
                .await
                .map_err(|_| "Não foi possível validar o login do usuário.".to_string())?
                .is_some();

            if !existe_config {
                return Err("Não foi possível validar o login do usuário.".to_string());
            }
        }

        let Some(usuario) = usuario::Entity::find()
            .filter(usuario::Column::Login.eq(login.login.to_uppercase()))
            .one(&self.db)
            .await
            .map_err(|_| "Não foi possível obter os dados do usuário.".to_string())?
        else {
            return Err("Não foi possível obter os dados do usuário.".to_string());
        };

        let senha_ok = validar_senha_pdv_scrypt(&login.senha, &usuario.senha);
        if !senha_ok {
            return Err("Usuário ou senha inválidos.".to_string());
        }

        if usuario.status == "B" {
            return Err("Login bloqueado. Contate o Administrador.".to_string());
        }

        if validar_acao {
            if let Some(acao) = login.acao.as_deref() {
                if !self.usuario_tem_permissao(usuario.id, acao).await {
                    return Err("Usuário não autorizado.".to_string());
                }
            }
        } else if !self.tem_alguma_permissao(usuario.id).await {
            return Err("Usuário não autorizado.".to_string());
        }

        Ok(usuario)
    }

    async fn tem_alguma_permissao(&self, usuario_id: i32) -> bool {
        usuario_permissao::Entity::find()
            .filter(usuario_permissao::Column::UsuarioId.eq(usuario_id))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .is_some()
    }

    async fn usuario_tem_permissao(&self, usuario_id: i32, acao: &str) -> bool {
        let Some(permissao) = usuario_permissao::Entity::find()
            .filter(usuario_permissao::Column::UsuarioId.eq(usuario_id))
            .one(&self.db)
            .await
            .ok()
            .flatten()
        else {
            return false;
        };

        let chave = acao.trim().to_ascii_lowercase().replace('-', "_");
        match chave.as_str() {
            "cancela_venda_aberta" | "cancela_cupom_aberto" => permissao.cancela_venda_aberta,
            "cancela_venda_fechada" | "cancela_cupom_fechado" => permissao.cancela_venda_fechada,
            "cancela_item" => permissao.cancela_item,
            "desconto_item" => permissao.desconto_item,
            "desconto_fechamento" => permissao.desconto_fechamento,
            "desconto_fechamento_pv" => permissao.desconto_fechamento_pv,
            "acrescimo_fechamento" => permissao.acrescimo_fechamento,
            "acrescimo_item" => permissao.acrescimo_item,
            "acrescimo_fechamento_pv" => permissao.acrescimo_fechamento_pv,
            "cliente_limite" => permissao.cliente_limite,
            "cliente_bloqueado" => permissao.cliente_bloqueado,
            "cliente_forma_pagamento" => permissao.cliente_forma_pagamento,
            "sangria" => permissao.sangria,
            "suprimento" => permissao.suprimento,
            "abertura_turno" => permissao.abertura_turno,
            "fechamento_turno" => permissao.fechamento_turno,
            "reabertura_turno" => permissao.reabertura_turno,
            "afericao" => permissao.afericao,
            "lista_todos_abastecimentos" => permissao.lista_todos_abastecimentos,
            "operacoes_tef" => permissao.operacoes_tef,
            "limite_desconto_acrescimo" => permissao.limite_desconto_acrescimo,
            "sangria_lancamento_saida" => permissao.sangria_lancamento_saida,
            "desmembramento" => permissao.desmembramento,
            "libera_troco_maximo" => permissao.libera_troco_maximo,
            "prmidentificacao" => Some("T".to_string()),
            _ => {
                error!("Ação de permissão não mapeada: {}", acao);
                Some("F".to_string())
            }
        }
        .as_deref()
        == Some("T")
    }
}

fn validar_senha_pdv_scrypt(senha_digitada: &str, hash_armazenado: &str) -> bool {
    let partes: Vec<&str> = hash_armazenado.split('$').collect();
    if partes.len() != 6 || partes[0] != "scrypt" {
        return false;
    }

    let Ok(n) = partes[1].parse::<u32>() else {
        return false;
    };
    let Ok(r) = partes[2].parse::<u32>() else {
        return false;
    };
    let Ok(p) = partes[3].parse::<u32>() else {
        return false;
    };
    let Ok(salt) = hex::decode(partes[4]) else {
        return false;
    };
    let Ok(esperado) = hex::decode(partes[5]) else {
        return false;
    };

    // N no hash vem como valor real (ex: 16384), mas o crate pede log2(N).
    if !n.is_power_of_two() {
        return false;
    }
    let log_n = n.ilog2() as u8;

    let Ok(params) = Params::new(log_n, r, p, esperado.len()) else {
        return false;
    };

    let mut derivado = vec![0_u8; esperado.len()];
    if scrypt(senha_digitada.as_bytes(), &salt, &params, &mut derivado).is_err() {
        return false;
    }

    timing_safe_equal(&derivado, &esperado)
}

fn timing_safe_equal(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut diff = 0u8;
    for (l, r) in left.iter().zip(right.iter()) {
        diff |= l ^ r;
    }

    diff == 0
}
