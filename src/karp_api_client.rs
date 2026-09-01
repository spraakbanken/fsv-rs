pub mod errors;
pub mod search;
pub mod types;

#[derive(Clone, Debug)]
pub struct KarpRedApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl Default for KarpRedApiClient {
    fn default() -> Self {
        Self::new()
    }
}

const KARP_RED_API_URL: &str = "https://spraakbanken4.it.gu.se/karp/v7";

impl KarpRedApiClient {
    pub fn new() -> Self {
        Self {
            base_url: KARP_RED_API_URL.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

pub struct RequestBuilder(pub reqwest::RequestBuilder);

impl KarpRedApiClient {
    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, &u);

        // Add in our authentication.
        // req = req.bearer_auth(&self.token);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = body {
            req = req.body(body);
        }

        Ok(RequestBuilder(req))
    }

    pub fn search(&self) -> search::Search {
        search::Search::new(self.clone())
    }
}
