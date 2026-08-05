pub fn api_base_url(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.ends_with("/api") {
        trimmed.to_string()
    } else {
        format!("{}/api", trimmed)
    }
}

pub fn sync_send_url(raw: &str, tipo: &str) -> String {
    format!("{}/pdv/sync?tipo={}", api_base_url(raw), tipo)
}
