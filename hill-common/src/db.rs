use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::info;

// Helper function to decrypt XOR-obfuscated credentials at runtime
fn decrypt_credential(encrypted: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = encrypted.iter().map(|&b| b ^ key).collect();
    String::from_utf8(decrypted).unwrap_or_default()
}

pub async fn establish_connection(db_ip: &str, db_port: &str) -> Result<DatabaseConnection, DbErr> {
    let db_ip = if db_ip.is_empty() { "localhost" } else { db_ip };
    let db_port = if db_port.is_empty() { "5455" } else { db_port };

    // XOR key used for obfuscation
    const XOR_KEY: u8 = 0x5A;

    // XOR-obfuscated bytes for "postgres"
    const ENC_USER: &[u8] = &[42, 37, 61, 46, 40, 63, 41];
    // XOR-obfuscated bytes for "H*9E9x3JlHdi"
    const ENC_PASS: &[u8] = &[18, 112, 99, 23, 99, 34, 105, 16, 54, 18, 30, 19];

    let db_user = decrypt_credential(ENC_USER, XOR_KEY);
    let db_pass = decrypt_credential(ENC_PASS, XOR_KEY);

    let database_url = format!(
        "postgres://{}:{}@{}:{}/hill",
        db_user, db_pass, db_ip, db_port
    );

    info!("Conectando ao banco de dados PostgreSQL em {}:{}...", db_ip, db_port);

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await?;

    info!("Conexão com o banco de dados estabelecida com sucesso.");
    Ok(db)
}

