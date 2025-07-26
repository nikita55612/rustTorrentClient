use crate::error::Result;
use crate::tracker::{
    announce::{AnnounceRequestParams, HttpAnnounceResponse},
    transport::TrackerTransportLayer,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName},
    Client,
};
use std::time::Duration;
use std::{future::Future, pin::Pin};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct HttpTrackerTransport {
    client: Client,
    headers: HeaderMap,
    timeout: Duration,
}

impl HttpTrackerTransport {
    pub fn builder() -> HttpTrackerTransportBuilder {
        HttpTrackerTransportBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct HttpTrackerTransportBuilder {
    client: Option<Client>,
    headers: HeaderMap,
    timeout: Duration,
}

impl HttpTrackerTransportBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            headers: HeaderMap::default(),
            timeout: DEFAULT_TIMEOUT,
        }
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn add_header<K: IntoHeaderName>(mut self, key: K, value: &str) -> Self {
        if let Ok(v) = HeaderValue::from_str(value) {
            self.headers.append(key, v);
        }
        self
    }

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn build(self) -> HttpTrackerTransport {
        HttpTrackerTransport {
            client: self.client.unwrap_or_default(),
            headers: self.headers,
            timeout: self.timeout,
        }
    }
}

impl TrackerTransportLayer for HttpTrackerTransport {
    fn announce<'a>(
        &'a self,
        url: &'a str,
        params: &'a AnnounceRequestParams,
    ) -> Pin<Box<dyn Future<Output = Result<HttpAnnounceResponse>> + Send + 'a>> {
        Box::pin(async move {
            let query = params.to_query_string();
            let full_url = format!("{url}?{query}");

            let response = self
                .client
                .get(&full_url)
                .headers(self.headers.clone())
                .timeout(self.timeout)
                .send()
                .await?;

            let bytes = response.bytes().await?;
            HttpAnnounceResponse::from_bytes(&bytes)?.check_failure_reason()
        })
    }
}
