use crate::error::Result;
use crate::tracker::announce::{AnnounceRequestParams, HttpAnnounceResponse};
use std::{future::Future, pin::Pin};

pub trait TrackerTransportLayer {
    fn announce<'a>(
        &'a self,
        url: &'a str,
        params: &'a AnnounceRequestParams,
    ) -> Pin<Box<dyn Future<Output = Result<HttpAnnounceResponse>> + Send + 'a>>;
}
