use super::announce::{AnnounceRequestParams, HttpAnnounceResponse};
use crate::error::Result;
use crate::tracker::transport::TrackerTransportLayer;
use std::sync::Arc;

#[derive(Clone)]
pub struct TrackerClient {
    url: String,
    transport: Arc<dyn TrackerTransportLayer>,
}

impl TrackerClient {
    pub fn new(url: impl Into<String>, transport: Arc<dyn TrackerTransportLayer>) -> Self {
        Self {
            url: url.into(),
            transport,
        }
    }

    pub async fn announce(
        &self,
        params: &AnnounceRequestParams<'_>,
    ) -> Result<HttpAnnounceResponse> {
        self.transport.announce(&self.url, params).await
    }
}
