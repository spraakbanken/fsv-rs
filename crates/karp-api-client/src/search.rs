use crate::KarpRedApiClient;
use crate::errors;
use crate::types;

#[derive(Clone, Debug)]
pub struct Search {
    client: KarpRedApiClient,
}

impl Search {
    pub fn new(client: KarpRedApiClient) -> Self {
        Self { client }
    }

    #[tracing::instrument]
    pub async fn query(&self, resources: &str) -> Result<types::SearchQuery, errors::Error> {
        let req = self.client.client.request(
            reqwest::Method::GET,
            format!("{}/query/{}", self.client.base_url, resources),
        );
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                errors::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text, err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(errors::Error::Server { body: text, status })
        }
    }
}
