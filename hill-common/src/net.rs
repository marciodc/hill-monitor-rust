use reqwest::Client;
use std::time::Duration;
use tracing::debug;

#[derive(Clone)]
pub struct HttpConn {
    client: Client,
}

impl Default for HttpConn {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpConn {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn post_json_servidor(
        &self,
        url: &str,
        payload_str: &str,
        token: &str,
    ) -> Result<String, reqwest::Error> {
        debug!("HTTP POST {}", url);
        debug!("HTTP POST payload: {}", payload_str);

        let response = self
            .client
            .post(url)
            .header("Authorization", token)
            .header("Content-Type", "application/json")
            .body(payload_str.to_string())
            .send()
            .await?;

        let response = response.error_for_status()?;
        let body = response.text().await?;
        debug!("HTTP POST response body: {}", body);
        Ok(body)
    }

    pub async fn get_json_servidor(
        &self,
        url: &str,
        token: &str,
    ) -> Result<String, reqwest::Error> {
        debug!("HTTP GET {}", url);

        let response = self
            .client
            .get(url)
            .header("Authorization", token)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = response.error_for_status()?;
        let body = response.text().await?;
        debug!("HTTP GET response body: {}", body);
        Ok(body)
    }
}
