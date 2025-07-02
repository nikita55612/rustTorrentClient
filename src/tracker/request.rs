/// https://wiki.theory.org/BitTorrentSpecification#Tracker_Request_Parameters
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct TrackerRequest {
    pub info_hash: String,
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub compact: Option<u8>,
    pub no_peer_id: Option<u8>,
    pub event: Option<String>,
    pub ip: Option<String>,
    pub numwant: Option<u32>,
    pub key: Option<String>,
    pub tracker_id: Option<String>,
}

impl TrackerRequest {
    pub fn builder() -> TrackerRequestBuilder {
        TrackerRequestBuilder::new()
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        params.push(format!("info_hash={}", self.info_hash));
        params.push(format!("peer_id={}", self.peer_id));
        params.push(format!("port={}", self.port));
        params.push(format!("uploaded={}", self.uploaded));
        params.push(format!("downloaded={}", self.downloaded));
        params.push(format!("left={}", self.left));

        if let Some(val) = self.compact {
            params.push(format!("compact={}", val));
        }

        if let Some(val) = self.no_peer_id {
            params.push(format!("no_peer_id={}", val));
        }

        if let Some(ref val) = self.event {
            params.push(format!("event={}", val));
        }

        if let Some(ref val) = self.ip {
            params.push(format!("ip={}", val));
        }

        if let Some(val) = self.numwant {
            params.push(format!("numwant={}", val));
        }

        if let Some(ref val) = self.key {
            params.push(format!("key={}", val));
        }

        if let Some(ref val) = self.tracker_id {
            params.push(format!("trackerid={}", val));
        }

        params.join("&")
    }
}

#[derive(Debug, Default)]
pub struct TrackerRequestBuilder {
    info_hash: Option<String>,
    peer_id: Option<String>,
    port: Option<u16>,
    uploaded: Option<u64>,
    downloaded: Option<u64>,
    left: Option<u64>,
    compact: Option<u8>,
    no_peer_id: Option<u8>,
    event: Option<String>,
    ip: Option<String>,
    numwant: Option<u32>,
    key: Option<String>,
    tracker_id: Option<String>,
}

impl TrackerRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info_hash(mut self, value: String) -> Self {
        self.info_hash = Some(value);
        self
    }

    pub fn peer_id(mut self, value: String) -> Self {
        self.peer_id = Some(value);
        self
    }

    pub fn port(mut self, value: u16) -> Self {
        self.port = Some(value);
        self
    }

    pub fn uploaded(mut self, value: u64) -> Self {
        self.uploaded = Some(value);
        self
    }

    pub fn downloaded(mut self, value: u64) -> Self {
        self.downloaded = Some(value);
        self
    }

    pub fn left(mut self, value: u64) -> Self {
        self.left = Some(value);
        self
    }

    pub fn compact(mut self, value: u8) -> Self {
        self.compact = Some(value);
        self
    }

    pub fn no_peer_id(mut self, value: u8) -> Self {
        self.no_peer_id = Some(value);
        self
    }

    pub fn event<S: Into<String>>(mut self, value: S) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn ip<S: Into<String>>(mut self, value: S) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn numwant(mut self, value: u32) -> Self {
        self.numwant = Some(value);
        self
    }

    pub fn key<S: Into<String>>(mut self, value: S) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn tracker_id<S: Into<String>>(mut self, value: S) -> Self {
        self.tracker_id = Some(value.into());
        self
    }

    pub fn build(self) -> Result<TrackerRequest> {
        Ok(TrackerRequest {
            info_hash: self
                .info_hash
                .ok_or_else(|| Error::Custom("info_hash is required".into()))?,
            peer_id: self
                .peer_id
                .ok_or_else(|| Error::Custom("peer_id is required".into()))?,
            port: self
                .port
                .ok_or_else(|| Error::Custom("port is required".into()))?,
            uploaded: self
                .uploaded
                .ok_or_else(|| Error::Custom("uploaded is required".into()))?,
            downloaded: self
                .downloaded
                .ok_or_else(|| Error::Custom("downloaded is required".into()))?,
            left: self
                .left
                .ok_or_else(|| Error::Custom("left is required".into()))?,
            compact: self.compact,
            no_peer_id: self.no_peer_id,
            event: self.event,
            ip: self.ip,
            numwant: self.numwant,
            key: self.key,
            tracker_id: self.tracker_id,
        })
    }
}
