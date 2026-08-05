#![recursion_limit = "256"]

mod backend_url;
mod config;
mod scheduler;
mod web;

use single_instance::SingleInstance;
use std::env;
use tracing::{error, info};
use tracing_subscriber::prelude::*;

fn normalize_log_level(level: &str) -> &'static str {
    match level.trim().to_ascii_uppercase().as_str() {
        "TRACE" => "trace",
        "DEBUG" => "debug",
        "WARN" | "WARNING" => "warn",
        "ERROR" => "error",
        "OFF" => "off",
        _ => "info",
    }
}

fn setup_logging(
    log_dir: &std::path::Path,
    console_level: &str,
) -> Option<tracing_appender::non_blocking::WorkerGuard> {

    // Daily rotating file appender (e.g. monitor.log.2026-08-04)
    // Note: tracing_appender rolling appends the date suffix automatically
    let file_appender = tracing_appender::rolling::daily(log_dir, "monitor.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(tracing_subscriber::EnvFilter::new("warn"));

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(tracing_subscriber::EnvFilter::new(normalize_log_level(console_level)));

    tracing_subscriber::registry()
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

    // 2. Resolve executable paths and load configuration (monitor.ini)
    let exe_dir = match env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())) {
        Some(dir) => dir,
        None => {
            eprintln!("Não foi possível determinar o diretório do executável.");
            std::process::exit(1);
        }
    };

    let ini_path = exe_dir.join("monitor.ini");

    // Create a dummy ini file if it doesn't exist for test purposes
    if !ini_path.exists() {
        eprintln!("Arquivo monitor.ini não encontrado. Criando arquivo de exemplo padrão.");
        let default_ini_content = "\
DB_IP=localhost
DB_PORTA=5432
LOG_SQL=F
LOG=INFO
LOG_TERMINAL=INFO
FABRICANTE=companytec
";
        if let Err(e) = std::fs::write(&ini_path, default_ini_content) {
            error!("Falha ao criar monitor.ini padrão: {:?}", e);
        }
    }

    let ini = match config::IniFile::read_from_file(&ini_path) {
        Ok(ini) => ini,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo INI: {:?}", e);
            std::process::exit(1);
        }
    };

    // 3. Setup Logging
    let log_dir = exe_dir.join("Log");
    let _guard = setup_logging(&log_dir, &ini.log_terminal);

    info!("Iniciando hill-monitor...");
    info!("Lendo arquivo de configuração de: {:?}", ini_path);
    info!("Configuração carregada com sucesso.");
    info!("DB IP: {}", ini.db_ip);
    info!("DB Porta: {}", ini.db_porta);
    info!("Log arquivo: WARN");
    info!("Log terminal: {}", ini.log_terminal);
    info!("SQL Log: {}", ini.log_sql);
    info!("Fabricante: {}", ini.fabricante);

    // 4. Connect to Database
    let log_sql = matches!(ini.log_sql.trim().to_ascii_uppercase().as_str(), "T" | "TRUE" | "1" | "YES" | "Y");

    let db_conn = match hill_common::db::establish_connection(&ini.db_ip, &ini.db_porta, log_sql).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Não foi possível estabelecer conexão com o banco de dados: {:?}", e);
            return;
        }
    };

    // 5. Initialize Concentrador (Serial Port & Scheduler)
    let config_helper = hill_common::config_helper::ConfigHelper::new(db_conn.clone());
    let serial_port = config_helper
        .get_parametro("CONCENTRADOR_Porta", None)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "COM1".to_string());

    info!("Inicializando comunicação com o concentrador serial na porta: {}", serial_port);
    let com = hill_concentrador::com::ConcentradorCom::new(&serial_port);
    let op = hill_concentrador::operation::ConcentradorOperacao::new(com, &ini.fabricante);
    let concentrador_scheduler = hill_concentrador::scheduler::ConcentradorScheduler::new(op, db_conn.clone());
    concentrador_scheduler.start();

    // 6. Start Monitor Schedulers (Atualizacao, Contingencia, Envio)
    let monitor_schedulers = scheduler::MonitorSchedulers::new(db_conn.clone());
    monitor_schedulers.start();

    // 7. Start HTTP Web Server using Axum on the local machine default port
    let app = web::create_router(db_conn);
    let bind_addr = "0.0.0.0:5000";

    info!("Servidor Web sendo iniciado em: {}", bind_addr);

    let listener = match tokio::net::TcpListener::bind(bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            error!(
                "Erro ao vincular listener TCP para o Servidor Web em {}: {:?}",
                bind_addr, e
            );
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
