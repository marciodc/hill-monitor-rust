use chrono::Datelike;
use hill_common::certificado::{descriptografar_pfx_base64, descriptografar_texto_utf8};
use hill_common::config_helper::ConfigHelper;
use hill_common::entity::venda;
use hill_nfe::AcBrNfe;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait,
    PaginatorTrait, QueryFilter,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{Duration, interval};
use tracing::{error, info, warn};

// Estruturas auxiliares para analisar respostas INI do ACBr
struct StatusServicoResposta {
    c_stat: i32,
}

struct ConsultaNFeResposta {
    c_stat: i32,
    n_prot: String,
    x_motivo: String,
}

struct InutilizarNFeResposta {
    c_stat: i32,
    n_prot: String,
    dh_recbto: String,
    x_motivo: String,
}

struct CancelamentoNFeResposta {
    c_stat: i32,
    n_prot: String,
    dh_recbto: String,
    xml: String,
}

#[allow(dead_code)]
struct EnvioRetornoResposta {
    envio_c_stat: i32,
    retorno_c_stat: i32,
    retorno_protocolo: String,
    retorno_dh_recbto: String,
    retorno_x_motivo: String,
}

fn parse_status_servico(ini_str: &str) -> Option<StatusServicoResposta> {
    let ini = ini::Ini::load_from_str(ini_str).ok()?;
    let sec = ini.section(Some("Status"))?;
    let c_stat = sec.get("CStat")?.parse().ok()?;
    Some(StatusServicoResposta { c_stat })
}

fn parse_consulta_nfe(ini_str: &str) -> Option<ConsultaNFeResposta> {
    let ini = ini::Ini::load_from_str(ini_str).ok()?;
    let sec = ini.section(Some("Consulta"))?;
    let c_stat = sec.get("CStat")?.parse().ok()?;
    let n_prot = sec.get("NProt").unwrap_or("").to_string();
    let x_motivo = sec.get("XMotivo").unwrap_or("").to_string();
    Some(ConsultaNFeResposta {
        c_stat,
        n_prot,
        x_motivo,
    })
}

fn parse_inutilizar_nfe(ini_str: &str) -> Option<InutilizarNFeResposta> {
    let ini = ini::Ini::load_from_str(ini_str).ok()?;
    let sec = ini.section(Some("Inutilizacao"))?;
    let c_stat = sec.get("CStat")?.parse().ok()?;
    let n_prot = sec.get("NProt").unwrap_or("").to_string();
    let dh_recbto = sec.get("DhRecbto").unwrap_or("").to_string();
    let x_motivo = sec.get("XMotivo").unwrap_or("").to_string();
    Some(InutilizarNFeResposta {
        c_stat,
        n_prot,
        dh_recbto,
        x_motivo,
    })
}

fn parse_cancelamento_nfe(ini_str: &str) -> Option<CancelamentoNFeResposta> {
    let ini = ini::Ini::load_from_str(ini_str).ok()?;
    let sec = ini.section(Some("Cancelamento"))?;
    let c_stat = sec.get("CStat")?.parse().ok()?;
    let n_prot = sec.get("nProt").unwrap_or("").to_string();
    let dh_recbto = sec.get("DhRecbto").unwrap_or("").to_string();
    let xml = sec.get("XML").unwrap_or("").to_string();
    Some(CancelamentoNFeResposta {
        c_stat,
        n_prot,
        dh_recbto,
        xml,
    })
}

fn parse_envio_retorno(ini_str: &str) -> Option<EnvioRetornoResposta> {
    let ini = ini::Ini::load_from_str(ini_str).ok()?;
    let envio_c_stat = ini
        .section(Some("Envio"))
        .and_then(|s| s.get("CStat"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let (retorno_c_stat, retorno_protocolo, retorno_dh_recbto, retorno_x_motivo) =
        if let Some(sec) = ini.section(Some("Retorno")) {
            let c_stat = sec.get("CStat").and_then(|v| v.parse().ok()).unwrap_or(0);
            let prot = sec.get("Protocolo").unwrap_or("").to_string();
            let dh = sec.get("DhRecbto").unwrap_or("").to_string();
            let mot = sec.get("XMotivo").unwrap_or("").to_string();
            (c_stat, prot, dh, mot)
        } else {
            (0, String::new(), String::new(), String::new())
        };

    Some(EnvioRetornoResposta {
        envio_c_stat,
        retorno_c_stat,
        retorno_protocolo,
        retorno_dh_recbto,
        retorno_x_motivo,
    })
}

async fn setup_acbr_nfe(
    config_helper: &ConfigHelper,
    pdv_config: &hill_common::entity::Configuracao,
) -> Result<AcBrNfe, String> {
    let lib_name = if cfg!(target_os = "windows") {
        "ACBrNFe64.dll"
    } else {
        "libacbrnfe64.so"
    };

    let mut lib_path = lib_name.to_string();
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let local_lib = parent.join(lib_name);
            if local_lib.exists() {
                lib_path = local_lib.to_string_lossy().into_owned();
            }
        }
    }

    let cert_pfx_protegido = config_helper
        .get_parametro("NF_CertificadoDadosPFX", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let cert_pfx = if cert_pfx_protegido.is_empty() {
        String::new()
    } else {
        descriptografar_pfx_base64(&cert_pfx_protegido)
            .map_err(|e| format!("Erro ao ler NF_CertificadoDadosPFX: {e}"))?
    };

    let cert_senha_protegida = config_helper
        .get_parametro("NF_CertificadoSenha", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let cert_senha = if cert_senha_protegida.is_empty() {
        String::new()
    } else {
        descriptografar_texto_utf8(&cert_senha_protegida)
            .map_err(|e| format!("Erro ao ler NF_CertificadoSenha: {e}"))?
    };

    let token_id = config_helper
        .get_parametro("NFe_IdToken", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let token_csc = config_helper
        .get_parametro("NFe_Token", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let tipo_ambiente = config_helper
        .get_parametro("NFe_TipoAmbiente", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let nfe_csrt = config_helper
        .get_parametro("NF_CSRT", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let nfe_id_csrt = config_helper
        .get_parametro("NF_IdCSRT", Some(pdv_config.id))
        .await
        .unwrap_or(None)
        .unwrap_or_default();

    let nfe = AcBrNfe::new(&lib_path, "[Memory]", &cert_senha)
        .map_err(|e| format!("Erro ao carregar ACBr NFe: {:?}", e))?;

    let base_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let base_dir_str = base_dir.to_string_lossy().into_owned();

    let sets = [
        ("Principal", "TipoResposta", "0"),
        ("Principal", "Codificacao", "0"),
        ("Principal", "LogNivel", "1"),
        ("Principal", "LogPath", &format!("{}/Log", base_dir_str)),
        ("Sistema", "Nome", "HillPDV"),
        ("Sistema", "Versao", "1.0"),
        ("Sistema", "Data", "01/12/2023"),
        ("Sistema", "Descricao", "Hill - Ponto de Vendas"),
        ("SoftwareHouse", "CNPJ", "47607257000170"),
        ("SoftwareHouse", "RazaoSocial", "Hill Tecnologia Ltda"),
        ("SoftwareHouse", "NomeFantasia", "Hill Tecnologia"),
        ("SoftwareHouse", "WebSite", "www.hilltecnologia.com.br"),
        ("SoftwareHouse", "Email", "contato@hilltecnologia.com.br"),
        ("SoftwareHouse", "Telefone", "82991741328"),
        (
            "SoftwareHouse",
            "Responsavel",
            "José Valdomiro da Silva Santos",
        ),
        ("Emissor", "CNPJ", pdv_config.cnpj.as_deref().unwrap_or("")),
        (
            "Emissor",
            "RazaoSocial",
            pdv_config.razao_social.as_deref().unwrap_or(""),
        ),
        (
            "Emissor",
            "NomeFantasia",
            pdv_config.nome_fantasia.as_deref().unwrap_or(""),
        ),
        (
            "Emissor",
            "Telefone",
            pdv_config.fone.as_deref().unwrap_or(""),
        ),
        ("DFe", "SSLCryptLib", "3"),
        ("DFe", "SSLHttpLib", "3"),
        ("DFe", "SSLXmlSignLib", "4"),
        ("DFe", "DadosPFX", &cert_pfx),
        ("DFe", "Senha", &cert_senha),
        ("DFe", "UF", pdv_config.uf.as_deref().unwrap_or("")),
        ("DFe", "TimeZoneModo", "0"),
        ("DFe", "VerificarValidade", "1"),
        ("NFe", "IdCSC", &token_id),
        ("NFe", "CSC", &token_csc),
        (
            "NFe",
            "Ambiente",
            if tipo_ambiente == "1" { "0" } else { "1" },
        ),
        ("NFe", "SalvarWS", "0"),
        ("NFe", "Timeout", "5000"),
        ("NFe", "TimeoutPorThread", "100"),
        ("NFe", "Visualizar", "0"),
        ("NFe", "AjustaAguardaConsultaRet", "0"),
        ("NFe", "AguardarConsultaRet", "100"),
        ("NFe", "IntervaloTentativas", "1000"),
        ("NFe", "Tentativas", "5"),
        ("NFe", "SSLType", "5"),
        ("NFe", "PathSalvar", &format!("{}/NFCe", base_dir_str)),
        (
            "NFe",
            "PathSchemas",
            &format!("{}/Schemas/NFe", base_dir_str),
        ),
        ("NFe", "SalvarArq", "1"),
        ("NFe", "SepararPorCNPJ", "1"),
        ("NFe", "SepararPorModelo", "1"),
        ("NFe", "SepararPorAno", "1"),
        ("NFe", "SepararPorMes", "1"),
        ("NFe", "SepararPorDia", "1"),
        ("NFe", "SalvarEvento", "1"),
        ("NFe", "SalvarApenasNFeProcessadas", "1"),
        ("NFe", "PathNFe", &format!("{}/NFCe", base_dir_str)),
        ("NFe", "PathInu", &format!("{}/NFCe/Inu", base_dir_str)),
        (
            "NFe",
            "PathEvento",
            &format!("{}/NFCe/Evento", base_dir_str),
        ),
        ("NFe", "IdCSRT", &nfe_id_csrt),
        ("NFe", "CSRT", &nfe_csrt),
    ];

    for &(sec, key, val) in &sets {
        let _ = nfe.config_gravar_val(sec, key, val);
    }

    let _ = nfe.config_gravar("");

    Ok(nfe)
}

async fn processa_nota_inconsistente(
    venda: venda::Model,
    nfe: &AcBrNfe,
    db: &DatabaseConnection,
    config_helper: &ConfigHelper,
    cnpj: &str,
) -> Result<(), DbErr> {
    if venda.nfe_tentativa_envio.unwrap_or(0) < 2 {
        let chave = venda.nfe_chave.as_deref().unwrap_or("");

        let nao_enviada = if chave.is_empty() {
            true
        } else {
            match nfe.consultar(chave, false) {
                Ok(resp_str) => {
                    let c_stat = parse_consulta_nfe(&resp_str).map(|r| r.c_stat).unwrap_or(0);
                    c_stat == 217
                }
                Err(_) => true,
            }
        };

        if nao_enviada {
            let ano = venda.nfe_data.map(|d| d.year() as i32).unwrap_or(0);
            let serie = venda.nfe_serie.unwrap_or(0);
            let numero = venda.nfe_numero.unwrap_or(0);

            let mut res_inutilizada = false;
            let mut dh_recbto = None;
            let mut n_prot = None;
            let mut rejeicao = None;

            if let Ok(resp_str) = nfe.inutilizar(
                cnpj,
                "INUTILIZACAO DEVIDO A PROBLEMAS TECNICOS",
                ano,
                65,
                serie,
                numero,
                numero,
            ) {
                if let Some(res) = parse_inutilizar_nfe(&resp_str) {
                    if res.c_stat == 999 {
                        return Ok(());
                    }
                    if res.c_stat == 102 && !res.n_prot.is_empty() {
                        res_inutilizada = true;
                        n_prot = Some(res.n_prot);
                        dh_recbto = chrono::NaiveDateTime::parse_from_str(
                            &res.dh_recbto,
                            "%d/%m/%Y %H:%M:%S",
                        )
                        .or_else(|_| {
                            chrono::NaiveDateTime::parse_from_str(
                                &res.dh_recbto,
                                "%Y-%m-%dT%H:%M:%S",
                            )
                        })
                        .ok();
                    } else {
                        rejeicao = Some(res.x_motivo);
                    }
                }
            }

            let new_tentativa = venda.nfe_tentativa_envio.unwrap_or(0) + 1;
            let mut active_venda: venda::ActiveModel = venda.into();
            if res_inutilizada {
                let _ = config_helper
                    .set_parametro(
                        "PDV_Contingencia",
                        "F",
                        Some(active_venda.pdv.clone().unwrap()),
                    )
                    .await;
                active_venda.nfe_aguardando_envio = ActiveValue::Set(Some("F".to_string()));
                active_venda.atualiza_retaguarda = ActiveValue::Set(Some("T".to_string()));
                active_venda.nfe_inutilizacao_data = ActiveValue::Set(dh_recbto);
                active_venda.nfe_inutilizacao_protocolo = ActiveValue::Set(n_prot);
                active_venda.nfe_inutilizada = ActiveValue::Set(Some("T".to_string()));
                active_venda.update(db).await?;
                return Ok(());
            } else {
                active_venda.nfe_rejeicao = ActiveValue::Set(rejeicao);
                active_venda.nfe_tentativa_envio = ActiveValue::Set(Some(new_tentativa));
                active_venda.update(db).await?;
                return Ok(());
            }
        }

        let mut autorizada = false;
        let mut protocolo = String::new();
        if !chave.is_empty() {
            if let Ok(resp_str) = nfe.consultar(chave, false) {
                if let Some(res) = parse_consulta_nfe(&resp_str) {
                    if res.c_stat == 100 || res.c_stat == 150 {
                        autorizada = true;
                        protocolo = res.n_prot;
                    }
                }
            }
        }

        if autorizada {
            let mut active_venda: venda::ActiveModel = venda.clone().into();
            active_venda.nfe_protocolo = ActiveValue::Set(Some(protocolo.clone()));
            let updated_venda = active_venda.update(db).await?;

            let numero = updated_venda.nfe_numero.unwrap_or(0);
            let mut cancelada = false;
            let mut dh_recbto = None;
            let mut c_prot = None;
            let mut xml = None;
            let mut rejeicao = None;

            if let Ok(resp_str) = nfe.cancelar(
                chave,
                "CANCELAMENTO DEVIDO A PROBLEMAS TECNICOS",
                cnpj,
                numero,
            ) {
                if let Some(res) = parse_cancelamento_nfe(&resp_str) {
                    if res.c_stat == 135 {
                        cancelada = true;
                        c_prot = Some(res.n_prot);
                        xml = Some(res.xml);
                        dh_recbto = chrono::NaiveDateTime::parse_from_str(
                            &res.dh_recbto,
                            "%d/%m/%Y %H:%M:%S",
                        )
                        .or_else(|_| {
                            chrono::NaiveDateTime::parse_from_str(
                                &res.dh_recbto,
                                "%Y-%m-%dT%H:%M:%S",
                            )
                        })
                        .ok();
                    } else {
                        rejeicao = Some(format!("RETORNO {}", res.c_stat));
                    }
                }
            }

            let mut active_venda: venda::ActiveModel = updated_venda.into();
            if cancelada {
                let _ = config_helper
                    .set_parametro(
                        "PDV_Contingencia",
                        "F",
                        Some(active_venda.pdv.clone().unwrap()),
                    )
                    .await;
                active_venda.nfe_cancelada = ActiveValue::Set(Some("T".to_string()));
                active_venda.nfe_cancelamento_data = ActiveValue::Set(dh_recbto);
                active_venda.nfe_cancelamento_motivo =
                    ActiveValue::Set(Some("CANCELAMENTO DEVIDO A PROBLEMAS TECNICOS".to_string()));
                active_venda.nfe_cancelamento_protocolo = ActiveValue::Set(c_prot);
                active_venda.nfe_cancelamento_xml = ActiveValue::Set(xml);
                active_venda.nfe_aguardando_envio = ActiveValue::Set(Some("F".to_string()));
                active_venda.atualiza_retaguarda = ActiveValue::Set(Some("T".to_string()));
            } else {
                active_venda.nfe_rejeicao = ActiveValue::Set(rejeicao);
                active_venda.nfe_aguardando_envio = ActiveValue::Set(Some("F".to_string()));
                active_venda.atualiza_retaguarda = ActiveValue::Set(Some("T".to_string()));
            }
            active_venda.update(db).await?;
        }
    }
    Ok(())
}

async fn processa_nota_contingencia(
    venda: venda::Model,
    nfe: &AcBrNfe,
    db: &DatabaseConnection,
    config_helper: &ConfigHelper,
) -> Result<(), DbErr> {
    if venda.nfe_tentativa_envio.unwrap_or(0) > 2 {
        return Ok(());
    }

    let _ = nfe.limpar_lista();
    let _ = nfe.carregar_xml(venda.nfe_xml.as_deref().unwrap_or(""));

    let numero = venda.nfe_numero.unwrap_or(0);
    let resp_str = match nfe.enviar(numero, false, true, false) {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };

    let resp = match parse_envio_retorno(&resp_str) {
        Some(r) => r,
        None => return Ok(()),
    };

    if resp.retorno_c_stat == 999 {
        return Ok(());
    }

    if resp.retorno_c_stat == 100 || resp.retorno_c_stat == 150 {
        let _ = config_helper
            .set_parametro("PDV_Contingencia", "F", Some(venda.pdv))
            .await;

        let _ = nfe.gravar_xml(0, "", "");
        let updated_xml = nfe.obter_xml(0).unwrap_or_default();

        let dh_recbto =
            chrono::NaiveDateTime::parse_from_str(&resp.retorno_dh_recbto, "%d/%m/%Y %H:%M:%S")
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(
                        &resp.retorno_dh_recbto,
                        "%Y-%m-%dT%H:%M:%S",
                    )
                })
                .ok();

        let mut active_venda: venda::ActiveModel = venda.into();
        active_venda.nfe_xml = ActiveValue::Set(Some(updated_xml));
        active_venda.nfe_protocolo = ActiveValue::Set(Some(resp.retorno_protocolo));
        active_venda.nfe_data = ActiveValue::Set(dh_recbto);
        active_venda.nfe_aguardando_envio = ActiveValue::Set(Some("F".to_string()));
        active_venda.atualiza_retaguarda = ActiveValue::Set(Some("T".to_string()));
        active_venda.sincronizado = ActiveValue::Set(Some("F".to_string()));
        active_venda.update(db).await?;
        return Ok(());
    }

    if resp.retorno_c_stat == 104 {
        let chave = venda.nfe_chave.as_deref().unwrap_or("");
        let resp_consulta_str = match nfe.consultar(chave, false) {
            Ok(s) => s,
            Err(_) => return Ok(()),
        };

        if let Some(res) = parse_consulta_nfe(&resp_consulta_str) {
            if res.c_stat == 100 {
                let _ = config_helper
                    .set_parametro("PDV_Contingencia", "F", Some(venda.pdv))
                    .await;

                let _ = nfe.gravar_xml(0, "", "");
                let updated_xml = nfe.obter_xml(0).unwrap_or_default();

                let mut active_venda: venda::ActiveModel = venda.into();
                active_venda.nfe_xml = ActiveValue::Set(Some(updated_xml));
                active_venda.nfe_protocolo = ActiveValue::Set(Some(resp.retorno_protocolo));
                active_venda.nfe_data = ActiveValue::Set(
                    chrono::NaiveDateTime::parse_from_str(
                        &resp.retorno_dh_recbto,
                        "%d/%m/%Y %H:%M:%S",
                    )
                    .or_else(|_| {
                        chrono::NaiveDateTime::parse_from_str(
                            &resp.retorno_dh_recbto,
                            "%Y-%m-%dT%H:%M:%S",
                        )
                    })
                    .ok(),
                );
                active_venda.nfe_aguardando_envio = ActiveValue::Set(Some("F".to_string()));
                active_venda.atualiza_retaguarda = ActiveValue::Set(Some("T".to_string()));
                active_venda.sincronizado = ActiveValue::Set(Some("F".to_string()));
                active_venda.update(db).await?;
                return Ok(());
            }

            let mut active_venda: venda::ActiveModel = venda.into();
            active_venda.nfe_rejeicao = ActiveValue::Set(Some(res.x_motivo));
            active_venda.update(db).await?;
            return Ok(());
        }
    }

    let new_tentativa = venda.nfe_tentativa_envio.unwrap_or(0) + 1;
    let mut active_venda: venda::ActiveModel = venda.into();
    active_venda.nfe_rejeicao = ActiveValue::Set(Some(resp.retorno_x_motivo));
    active_venda.nfe_tentativa_envio = ActiveValue::Set(Some(new_tentativa));
    active_venda.update(db).await?;
    Ok(())
}

pub struct ContingenciaScheduler {
    db: DatabaseConnection,
    running: Arc<AtomicBool>,
}

impl ContingenciaScheduler {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let running = self.running.clone();

        running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(600)); // Executa a cada 10 minutos (600 segundos) conforme original C#
            let config_helper = ConfigHelper::new(db.clone());
            info!("ContingenciaScheduler iniciado.");

            while running.load(Ordering::SeqCst) {
                interval.tick().await;

                let configuracoes = match config_helper.list_configuracoes().await {
                    Ok(configs) => configs,
                    Err(e) => {
                        error!("Erro ao carregar PDVs para contingência: {:?}", e);
                        continue;
                    }
                };

                for configuracao in configuracoes {
                    // Contagem de vendas offline para processar
                    let offline_count = match venda::Entity::find()
                        .filter(venda::Column::Pdv.eq(configuracao.id))
                        .filter(
                            venda::Column::NfeTentativaEnvio
                                .is_null()
                                .or(venda::Column::NfeTentativaEnvio.lt(2)),
                        )
                        .filter(venda::Column::NfeAguardandoEnvio.eq("T"))
                        .filter(venda::Column::NfeOffline.eq("T"))
                        .filter(
                            venda::Column::NfeInconsistente
                                .ne("T")
                                .or(venda::Column::NfeInconsistente.is_null()),
                        )
                        .count(&db)
                        .await
                    {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Erro ao contar vendas offline: {:?}", e);
                            continue;
                        }
                    };

                    // Contagem de vendas inconsistentes para processar
                    let inconsistente_count = match venda::Entity::find()
                        .filter(venda::Column::Pdv.eq(configuracao.id))
                        .filter(
                            venda::Column::NfeTentativaEnvio
                                .is_null()
                                .or(venda::Column::NfeTentativaEnvio.lt(2)),
                        )
                        .filter(venda::Column::NfeAguardandoEnvio.eq("T"))
                        .filter(venda::Column::NfeInconsistente.eq("T"))
                        .count(&db)
                        .await
                    {
                        Ok(c) => c,
                        Err(e) => {
                            error!("Erro ao contar vendas inconsistentes: {:?}", e);
                            continue;
                        }
                    };

                    if offline_count == 0 && inconsistente_count == 0 {
                        continue;
                    }

                    info!(
                        "ContingenciaScheduler processando contingência para o PDV: {:?}",
                        configuracao.id
                    );

                    let nfe = match setup_acbr_nfe(&config_helper, &configuracao).await {
                        Ok(n) => n,
                        Err(e) => {
                            error!(
                                "Falha ao configurar ACBr para PDV {:?}: {}",
                                configuracao.id, e
                            );
                            continue;
                        }
                    };

                    let status_resp_str = match nfe.status_servico() {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("Falha ao consultar status de serviço: {:?}", e);
                            continue;
                        }
                    };

                    let status_cstat = parse_status_servico(&status_resp_str)
                        .map(|s| s.c_stat)
                        .unwrap_or(0);
                    if status_cstat != 107 {
                        info!(
                            "Status do serviço SEFAZ não está pronto (cStat = {}). Ignorando.",
                            status_cstat
                        );
                        continue;
                    }

                    // Processa notas inconsistentes
                    let vendas_inconsistentes = match venda::Entity::find()
                        .filter(venda::Column::Pdv.eq(configuracao.id))
                        .filter(
                            venda::Column::NfeTentativaEnvio
                                .is_null()
                                .or(venda::Column::NfeTentativaEnvio.lt(2)),
                        )
                        .filter(venda::Column::NfeAguardandoEnvio.eq("T"))
                        .filter(venda::Column::NfeInconsistente.eq("T"))
                        .all(&db)
                        .await
                    {
                        Ok(list) => list,
                        Err(e) => {
                            error!("Erro ao carregar vendas inconsistentes: {:?}", e);
                            Vec::new()
                        }
                    };

                    for v in vendas_inconsistentes {
                        let cnpj = configuracao.cnpj.as_deref().unwrap_or("");
                        let _ =
                            processa_nota_inconsistente(v, &nfe, &db, &config_helper, cnpj).await;
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }

                    // Processa notas offline
                    let vendas_offline = match venda::Entity::find()
                        .filter(venda::Column::Pdv.eq(configuracao.id))
                        .filter(
                            venda::Column::NfeTentativaEnvio
                                .is_null()
                                .or(venda::Column::NfeTentativaEnvio.lt(2)),
                        )
                        .filter(venda::Column::NfeAguardandoEnvio.eq("T"))
                        .filter(venda::Column::NfeOffline.eq("T"))
                        .filter(
                            venda::Column::NfeInconsistente
                                .ne("T")
                                .or(venda::Column::NfeInconsistente.is_null()),
                        )
                        .all(&db)
                        .await
                    {
                        Ok(list) => list,
                        Err(e) => {
                            error!("Erro ao carregar vendas offline: {:?}", e);
                            Vec::new()
                        }
                    };

                    for v in vendas_offline {
                        let _ = processa_nota_contingencia(v, &nfe, &db, &config_helper).await;
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
            info!("ContingenciaScheduler parado.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
