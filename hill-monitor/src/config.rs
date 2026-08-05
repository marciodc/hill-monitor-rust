use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct IniFile {
    pub db_ip: String,
    pub db_porta: String,
    pub log_sql: String,
    pub log: String,
    pub log_terminal: String,
    pub fabricante: String,
}

impl IniFile {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut ini = Self::default();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            // Ignore comments or empty lines
            if line.trim().starts_with(';') || line.trim().starts_with('#') || line.trim().is_empty() {
                continue;
            }

            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim();
                let val = val.trim();

                match key {
                    "DB_IP" => ini.db_ip = val.to_string(),
                    "DB_PORTA" => ini.db_porta = val.to_string(),
                    "LOG_SQL" => ini.log_sql = val.to_string(),
                    "LOG" => ini.log = val.to_string(),
                    "LOG_TERMINAL" => ini.log_terminal = val.to_string(),
                    "FABRICANTE" => ini.fabricante = val.to_string(),
                    _ => {}
                }
            }
        }

        Ok(ini)
    }
}
