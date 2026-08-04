use reqwest::Client;
use std::time::Duration;

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
        let response = self.client
            .post(url)
            .header("Authorization", token)
            .header("Content-Type", "application/json")
            .body(payload_str.to_string())
            .send()
            .await?;

        let response = response.error_for_status()?;
        response.text().await
    }

    pub async fn get_json_servidor(
        &self,
        url: &str,
        token: &str,
    ) -> Result<String, reqwest::Error> {
        let response = self.client
            .get(url)
            .header("Authorization", token)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let response = response.error_for_status()?;
        response.text().await
    }
}
