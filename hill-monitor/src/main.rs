#![recursion_limit = "256"]

mod config;
mod scheduler;
mod web;

use single_instance::SingleInstance;
use std::env;
use tracing::{error, info};
use tracing_subscriber::prelude::*;

fn setup_logging() -> Option<tracing_appender::non_blocking::WorkerGuard> {
    // Get executable directory
    let exe_path = env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;
    let log_dir = exe_dir.join("Log");

    // Daily rotating file appender (e.g. monitor.log.2026-08-04)
    // Note: tracing_appender rolling appends the date suffix automatically
    let file_appender = tracing_appender::rolling::daily(log_dir, "monitor.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(file_layer)
        .with(console_layer)
        .init();

    Some(guard)
}

#[tokio::main]
async fn main() {
    // 1. Single Instance Check
    let instance = SingleInstance::new("br.com.hilltecnologia.monitor")
        .expect("Falha ao inicializar verificação de instância única.");
    if !instance.is_single() {
        eprintln!("Já existe uma instância do aplicativo em execução.");
        std::process::exit(1);
    }

    // 2. Setup Logging
    let _guard = setup_logging();

    info!("Iniciando hill-monitor...");

    // 3. Load configuration (HillPDV.ini)
    let exe_dir = match env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())) {
        Some(dir) => dir,
        None => {
            error!("Não foi possível determinar o diretório do executável.");
            std::process::exit(1);
        }
    };

    let ini_path = exe_dir.join("HillPDV.ini");
    info!("Lendo arquivo de configuração de: {:?}", ini_path);

    // Create a dummy ini file if it doesn't exist for test purposes
    if !ini_path.exists() {
        info!("Arquivo HillPDV.ini não encontrado. Criando arquivo de exemplo padrão.");
        let default_ini_content = "\
PDV=00000000-0000-0000-0000-000000000000
DB_IP=localhost
DB_PORTA=5455
MONITOR_URL=http://127.0.0.1:5000
LOG=INFO
LOG_TERMINAL=INFO
FABRICANTE=companytec
";
        if let Err(e) = std::fs::write(&ini_path, default_ini_content) {
            error!("Falha ao criar HillPDV.ini padrão: {:?}", e);
        }
    }

    let ini = match config::IniFile::read_from_file(&ini_path) {
        Ok(ini) => {
            info!("Configuração carregada com sucesso.");
            info!("PDV UUID: {}", ini.pdv);
            info!("DB IP: {}", ini.db_ip);
            info!("DB Porta: {}", ini.db_porta);
            info!("Monitor URL: {}", ini.monitor_url);
            info!("Fabricante: {}", ini.fabricante);
            ini
        }
        Err(e) => {
            error!("Erro ao ler o arquivo INI: {:?}", e);
            std::process::exit(1);
        }
    };

    // 4. Connect to Database
    let db_conn = match hill_common::db::establish_connection(&ini.db_ip, &ini.db_porta).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Não foi possível estabelecer conexão com o banco de dados: {:?}", e);
            return;
        }
    };

    let pdv_uuid = uuid::Uuid::parse_str(&ini.pdv).unwrap_or_else(|_| uuid::Uuid::nil());

    // 5. Initialize Concentrador (Serial Port & Scheduler)
    let config_helper = hill_common::config_helper::ConfigHelper::new(db_conn.clone());
    let serial_port = config_helper
        .get_parametro("CONCENTRADOR_Porta", Some(pdv_uuid))
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "COM1".to_string());

    info!("Inicializando comunicação com o concentrador serial na porta: {}", serial_port);
    let com = hill_concentrador::com::ConcentradorCom::new(&serial_port);
    let op = hill_concentrador::operation::ConcentradorOperacao::new(com, &ini.fabricante);
    let concentrador_scheduler = hill_concentrador::scheduler::ConcentradorScheduler::new(op, db_conn.clone());
    concentrador_scheduler.start();

    // 6. Start Monitor Schedulers (Atualizacao, Contingencia, Envio)
    let monitor_schedulers = scheduler::MonitorSchedulers::new(db_conn.clone(), pdv_uuid);
    monitor_schedulers.start();

    // 7. Start HTTP Web Server using Axum
    let app = web::create_router(db_conn);

    // Clean URL string for binding
    let addr_str = ini
        .monitor_url
        .replace("http://", "")
        .replace("https://", "");

    info!("Servidor Web sendo iniciado em: {}", addr_str);

    let listener = match tokio::net::TcpListener::bind(&addr_str).await {
        Ok(l) => l,
        Err(e) => {
            error!("Erro ao vincular listener TCP para o Servidor Web na porta {}: {:?}", addr_str, e);
            concentrador_scheduler.stop();
            monitor_schedulers.stop();
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        error!("Erro na execução do Servidor Web Axum: {:?}", e);
    }

    concentrador_scheduler.stop();
    monitor_schedulers.stop();
    info!("Aplicação finalizada.");
}
